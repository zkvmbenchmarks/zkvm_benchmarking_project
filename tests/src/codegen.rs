use crate::env_adapters::CodeEnv;
use quote::{quote, ToTokens};
use std::{collections::HashMap, fs};
use syn::{parse_file, spanned::Spanned, Attribute, Expr, File, Item, ItemFn, Stmt};

pub struct CodeGenerator {
    env: Box<dyn CodeEnv>,
}

impl CodeGenerator {
    pub fn new(env: Box<dyn CodeEnv>) -> Self {
        Self { env }
    }

    pub fn generate_code(&self, input_path: &str, output_path: &str, host_output_path: &str) {
        self.generate_host_code(input_path, host_output_path);
        append_host_functions_to_syn_tree(input_path, host_output_path);
        let code = fs::read_to_string(input_path).expect("Failed to read input file");
        let syntax_tree: File = parse_file(&code).expect("Failed to parse Rust code");

        let transformed = self.transform(syntax_tree);

        fs::write(output_path, quote!(#transformed).to_string())
            .expect("Failed to write output file");
    }

    fn transform(&self, mut syntax_tree: syn::File) -> syn::File {
        let mut new_items = Vec::new();

        for import in self.env.import() {
            new_items.push(import);
        }

        for item in syntax_tree.items {
            match &item {
                syn::Item::Use(use_item) => {
                    if let syn::UseTree::Path(path) = &use_item.tree {
                        if path.ident == "crate"
                            && matches!(
                                &*path.tree,
                                syn::UseTree::Path(inner) if inner.ident == "env_adapters" &&
                                    matches!(
                                        &*inner.tree,
                                        syn::UseTree::Rename(rename) if rename.ident == "NotImplementedEnv"
                                    )
                            )
                        {
                            continue;
                        }
                    }

                    new_items.push(item);
                }
                syn::Item::Fn(func) if func.sig.ident == "main" => {
                    let mut transformed_func = func.clone();
                    self.transform_main_body(&mut transformed_func);
                    new_items.push(syn::Item::Fn(transformed_func));
                }
                _ => {
                    new_items.push(item);
                }
            }
        }

        syntax_tree.items = new_items;
        syntax_tree
    }

    fn transform_main_body(&self, func: &mut syn::ItemFn) {
        let mut transformed_stmts = Vec::new();

        for stmt in &func.block.stmts {
            match stmt {
                syn::Stmt::Local(local) => {
                    if let Some(init) = &local.init {
                        if let Some(transformed_stmt) = self.transform_env_expr(&init.expr) {
                            let mut new_local = local.clone();
                            new_local.init = Some(syn::LocalInit {
                                eq_token: init.eq_token.clone(),
                                expr: Box::new(syn::Expr::Verbatim(
                                    quote::quote!(#transformed_stmt),
                                )),
                                diverge: init.diverge.clone(),
                            });
                            transformed_stmts.push(syn::Stmt::Local(new_local));
                            continue;
                        }
                    }
                    transformed_stmts.push(stmt.clone());
                }

                syn::Stmt::Expr(expr, _) => {
                    if let Some(transformed_stmt) = self.transform_env_expr(expr) {
                        transformed_stmts.push(transformed_stmt);
                    } else {
                        transformed_stmts.push(stmt.clone());
                    }
                }

                _ => transformed_stmts.push(stmt.clone()),
            }
        }

        func.block.stmts = transformed_stmts;
    }

    fn transform_env_expr(&self, expr: &syn::Expr) -> Option<syn::Stmt> {
        if let syn::Expr::Call(call) = expr {
            if let syn::Expr::Path(path) = &*call.func {
                let segments = &path.path.segments;

                if segments.len() == 2 && segments[0].ident == "env" {
                    match segments[1].ident.to_string().as_str() {
                        "read" => {
                            if call.args.is_empty() {
                                return Some(self.env.read());
                            }
                        }
                        "commit" => {
                            if let Some(arg) = call.args.first() {
                                if let syn::Expr::Path(arg_path) = arg {
                                    let var_name = arg_path.path.segments[0].ident.to_string();
                                    let stmt = self.env.commit(&var_name);
                                    return Some(stmt);
                                } else if let syn::Expr::Reference(ref_expr) = arg {
                                    if let syn::Expr::Path(arg_path) = &*ref_expr.expr {
                                        let var_name = arg_path.path.segments[0].ident.to_string();
                                        let stmt = self.env.commit(&var_name);
                                        return Some(stmt);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        None
    }


    fn generate_host_code(
        &self,
        input_path: &str,
        host_file_path: &str,
    ) {
        let source_code = fs::read_to_string(input_path).expect("Failed to read the input file");
        let syntax_tree = parse_file(&source_code).expect("Failed to parse the input file");
        let main_function = syntax_tree
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(func) if func.sig.ident == "main" => Some(func),
                _ => None,
            })
            .expect("Main function not found");
    
        let source_lines: Vec<&str> = source_code.lines().collect();
        let assignments = extract_read_assignments(main_function, &source_lines);
    
        let env_code = self.env.generate_host_env(&assignments);
    
        let host_template = include_str!("../host_templates/risc_zero.rs");

        let mut generated_code = host_template.to_string();
        let assignment_lines = assignments.join("\n");
        generated_code = generated_code.replace("// INPUT_ASSIGNMENTS", &assignment_lines);
        generated_code = generated_code.replace("// ENVIRONMENT_BUILDER", &env_code);
    
        fs::write(host_file_path, generated_code).expect("Failed to write to the host file");
    }

}

/// Check if an attribute list contains #[host]
fn has_host_annotation(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.meta.path().is_ident("host"))
}

fn append_host_functions_to_syn_tree(input_path: &str, host_file_path: &str) {
    let source_code =
        fs::read_to_string(input_path).expect("Failed to read the input Rust source file");

    let syntax_tree = parse_file(&source_code).expect("Failed to parse the input Rust source file");

    let host_functions: Vec<ItemFn> = syntax_tree
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Fn(mut item_fn) if has_host_annotation(&item_fn.attrs) => {
                item_fn
                    .attrs
                    .retain(|attr| !attr.meta.path().is_ident("host"));
                Some(item_fn)
            }
            _ => None,
        })
        .collect();

    let mut host_syntax_tree: File = match fs::read_to_string(host_file_path) {
        Ok(content) => parse_file(&content).expect("Failed to parse the existing host file"),
        Err(_) => File {
            shebang: None,
            items: Vec::new(),
            attrs: Vec::new(),
        },
    };

    host_syntax_tree
        .items
        .extend(host_functions.into_iter().map(Item::Fn));

    let updated_code = format_syntax_tree(&host_syntax_tree);

    fs::write(host_file_path, updated_code).expect("Failed to write the updated host file");
}

fn format_syntax_tree(file: &File) -> String {
    let mut code = String::new();

    // Serialize each item in the syntax tree into its token stream
    for item in &file.items {
        code.push_str(&item.to_token_stream().to_string());
        code.push('\n');
    }

    code
}


/// Extract `env::read()` calls from the body of the `main` function and generate assignments
fn extract_read_assignments(main_function: &ItemFn, source_lines: &[&str]) -> Vec<String> {
    let mut assignments = Vec::new();
    let mut current_line = 0;
    let mut input_index = 1;

    for stmt in &main_function.block.stmts {
        while current_line < source_lines.len() && !source_lines[current_line].contains("env::read") {
            current_line += 1;
        }

        if let Stmt::Local(local) = stmt {
            // Check if the statement has an initializer expression
            if let Some(init) = &local.init {
                let expr = &init.expr;
                if is_env_read_call(expr) {
                    // Extract function name and arguments from the comment
                    if let Some((func_name, args)) = extract_function_from_comment(source_lines[current_line]) {
                        // Generate the assignment line
                        if args.is_empty() {
                            assignments.push(format!("let input{} = {}();", input_index, func_name));
                        } else {
                            assignments.push(format!("let input{} = {}({});", input_index, func_name, args));
                        }
                        input_index += 1;
                    }
                }
            }
        }

        current_line += 1;
    }

    assignments
}
/// Check if an expression is an `env::read()` call
fn is_env_read_call(expr: &Expr) -> bool {
    println!("Checking if expression is env::read() call: {}", expr.to_token_stream());
    if let Expr::Call(call_expr) = expr {
        if let Expr::Path(path) = call_expr.func.as_ref() {
            // Match the specific `env::read` path
            println!("{}", path.to_token_stream().to_string() == "env :: read");
            return path.to_token_stream().to_string() == "env :: read";
        }
    }
    println!("Not an env::read() call");
    false
}

/// Extract the function name and arguments from a comment
fn extract_function_from_comment(line: &str) -> Option<(String, String)> {
    // Look for a comment in the format `// #function_name(args)`
    if let Some(comment_start) = line.find("// #") {
        let comment = &line[comment_start + 4..];
        if let Some(open_paren_pos) = comment.find('(') {
            let func_name = &comment[..open_paren_pos].trim();
            let args = &comment[open_paren_pos + 1..comment.len() - 1].trim(); // Exclude closing ')'
            return Some((func_name.to_string(), args.to_string()));
        }
    }
    None
}