use crate::env_adapters::CodeEnv;
use quote::quote;
use std::fs;
use syn::{parse_file, File};

pub struct CodeGenerator {
    env: Box<dyn CodeEnv>,
}

impl CodeGenerator {
    pub fn new(env: Box<dyn CodeEnv>) -> Self {
        Self { env }
    }

    pub fn generate_code(&self, input_path: &str, output_path: &str) {
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
                                expr: Box::new(syn::Expr::Verbatim(quote::quote!(#transformed_stmt))),
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
}
