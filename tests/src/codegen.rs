use crate::env_adapters::CodeEnv;
use quote::{quote, ToTokens};
use std::{collections::HashMap, fs, path::Path};
use syn::{parse_file, spanned::Spanned, Attribute, Expr, File, Item, ItemFn, Stmt};
use toml;

pub struct CodeGenerator {
    env: Box<dyn CodeEnv>,
    saved_state: SavedState,
}

struct SavedState {
    host_output_dir: String,
    guest_output_dir: String,
    host_cargo_toml: String,
    guest_cargo_toml: String,
    host_files: HashMap<String, Vec<u8>>,
    guest_files: HashMap<String, Vec<u8>>,
}

impl CodeGenerator {
    pub fn new(env: Box<dyn CodeEnv>) -> Self {
        let saved_state = Self::save_initial_state(&env);
        Self { env, saved_state }
    }

    fn save_initial_state(env: &Box<dyn CodeEnv>) -> SavedState {
        let host_output_dir = env.get_host_output_dir();
        let guest_output_dir = env.get_guest_output_dir();
        let host_cargo_toml_path = env.get_host_cargo_toml_path();
        let guest_cargo_toml_path = env.get_guest_cargo_toml_path();

        let host_cargo_toml =
            fs::read_to_string(&host_cargo_toml_path).expect("Failed to read host Cargo.toml");
        let guest_cargo_toml =
            fs::read_to_string(&guest_cargo_toml_path).expect("Failed to read guest Cargo.toml");

        let host_files = Self::read_directory_files(&host_output_dir);
        let guest_files = Self::read_directory_files(&guest_output_dir);

        SavedState {
            host_output_dir,
            guest_output_dir,
            host_cargo_toml,
            guest_cargo_toml,
            host_files,
            guest_files,
        }
    }

    fn read_directory_files(dir: &str) -> HashMap<String, Vec<u8>> {
        let mut files = HashMap::new();
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let content = fs::read(&path).expect("Failed to read file");
                files.insert(file_name, content);
            }
        }
        files
    }

    pub fn reset(&self) {
        fs::remove_dir_all(&self.saved_state.host_output_dir)
            .expect("Failed to remove host output directory");
        fs::create_dir_all(&self.saved_state.host_output_dir)
            .expect("Failed to create host output directory");

        fs::remove_dir_all(&self.saved_state.guest_output_dir)
            .expect("Failed to remove guest output directory");
        fs::create_dir_all(&self.saved_state.guest_output_dir)
            .expect("Failed to create guest output directory");

        for (file_name, content) in &self.saved_state.host_files {
            let file_path = Path::new(&self.saved_state.host_output_dir).join(file_name);
            fs::write(file_path, content).expect("Failed to write host file");
        }

        for (file_name, content) in &self.saved_state.guest_files {
            let file_path = Path::new(&self.saved_state.guest_output_dir).join(file_name);
            fs::write(file_path, content).expect("Failed to write guest file");
        }

        let host_cargo_toml_path = self.env.get_host_cargo_toml_path();
        fs::write(host_cargo_toml_path, &self.saved_state.host_cargo_toml)
            .expect("Failed to write host Cargo.toml");

        let guest_cargo_toml_path = self.env.get_guest_cargo_toml_path();
        fs::write(guest_cargo_toml_path, &self.saved_state.guest_cargo_toml)
            .expect("Failed to write guest Cargo.toml");
    }

    pub fn generate_code(&self, input_dir: &str, output_dir: &str, host_output_dir: &str) {
        let dir_name = Path::new(input_dir)
            .file_name()
            .expect("Failed to get directory name")
            .to_str()
            .expect("Failed to convert directory name to string");

        let mut template_path = String::new();
        let mut toml_path = String::new();
        let mut additional_file_paths = Vec::new();

        for entry in fs::read_dir(input_dir).expect("Failed to read input directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name == format!("{}.rs", dir_name) {
                    template_path = path.to_str().unwrap().to_string();
                } else if file_name.ends_with(".toml") {
                    toml_path = path.to_str().unwrap().to_string();
                } else {
                    additional_file_paths.push(path.to_str().unwrap().to_string());
                }
            }
        }

        let host_output_path = Path::new(host_output_dir)
            .join("main.rs")
            .to_str()
            .unwrap()
            .to_string();
        self.generate_host_code(&template_path, &host_output_path);
        prepend_host_imports_to_syn_tree(&template_path, &host_output_path);
        append_host_functions_to_syn_tree(&template_path, &host_output_path);
        self.handle_precompiles(&template_path);
        self.handle_dependencies(&toml_path);

        let code = fs::read_to_string(&template_path).expect("Failed to read input file");
        let mut syntax_tree: File = parse_file(&code).expect("Failed to parse Rust code");

        strip_attribute(&mut syntax_tree, "precompile");
        let transformed = self.transform(syntax_tree);

        let output_path = Path::new(output_dir).join("main.rs");
        fs::write(output_path, quote!(#transformed).to_string())
            .expect("Failed to write output file");

        for path in additional_file_paths {
            self.copy_additional_files(&path);
        }
    }

    fn copy_additional_files(&self, input_path: &str) {
        let template_dir = Path::new(input_path).parent().unwrap();
        let binding = self.env.get_file_copy_destination();
        let destination_dir = Path::new(&binding);
        println!("Path: {}", destination_dir.display());

        for entry in fs::read_dir(template_dir).expect("Failed to read template directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let destination_path = destination_dir.join(file_name);
                fs::copy(&path, &destination_path).expect("Failed to copy file");
            }
        }
    }

    fn handle_dependencies(&self, toml_path: &str) {
        let toml_content = fs::read_to_string(toml_path).expect("Failed to read rsa.toml");
        let toml_value: toml::Value =
            toml::from_str(&toml_content).expect("Failed to parse rsa.toml");

        let host_dependencies = toml_value
            .get("host_dependencies")
            .expect("Missing host_dependencies");
        let guest_dependencies = toml_value
            .get("guest_dependencies")
            .expect("Missing guest_dependencies");

        let host_cargo_toml_path = self.env.get_host_cargo_toml_path();
        let guest_cargo_toml_path = self.env.get_guest_cargo_toml_path();

        self.update_cargo_toml(&host_cargo_toml_path, host_dependencies);
        self.update_cargo_toml(&guest_cargo_toml_path, guest_dependencies);
    }

    fn handle_precompiles(&self, input_path: &str) {
        let code = fs::read_to_string(input_path).expect("Failed to read input file");
        let syntax_tree: File = parse_file(&code).expect("Failed to parse Rust code");

        // Process items with #[precompile] attribute
        let precompile_items = process_items_with_attribute(
            syntax_tree.items,
            "precompile",
            true,  // strip_attribute
            false, // strip_item
        );

        let mut patches = HashMap::new();

        for item in precompile_items {
            if let syn::Item::Use(use_item) = item {
                if let syn::UseTree::Path(path) = &use_item.tree {
                    let crate_name = path.ident.to_string();
                    if let Some(patch) = self.env.get_available_patches().get(&crate_name) {
                        patches.insert(crate_name, patch.clone());
                    }
                }
            }
        }

        if !patches.is_empty() {
            let cargo_toml_path = self.env.get_workspace_cargo_toml_path();
            let mut cargo_toml_content = fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
            let mut cargo_toml_value: toml::Value = toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

            // Ensure the patch section exists
            let patch_section = cargo_toml_value.as_table_mut().unwrap().entry("patch").or_insert_with(|| {
                toml::Value::Table(toml::map::Map::new())
            });

            // Ensure the crates-io section exists within the patch section
            let crates_io_section = patch_section.as_table_mut().unwrap().entry("crates-io").or_insert_with(|| {
                toml::Value::Table(toml::map::Map::new())
            });

            if let toml::Value::Table(crates_io_table) = crates_io_section {
                for (crate_name, patch) in patches {
                    let patch_value: toml::Value = toml::from_str(&patch).expect("Failed to parse patch");
                    if let toml::Value::Table(patch_table) = patch_value {
                        for (key, value) in patch_table {
                            crates_io_table.insert(key, value);
                        }
                    }
                }
            }

            println!("Patched Cargo.toml: {:#?}", cargo_toml_value);

            cargo_toml_content = toml::to_string(&cargo_toml_value).expect("Failed to serialize Cargo.toml");
            fs::write(cargo_toml_path, cargo_toml_content).expect("Failed to write Cargo.toml");
        }
    }


    fn update_cargo_toml(&self, cargo_toml_path: &str, dependencies: &toml::Value) {
        let mut cargo_toml_content =
            fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
        let mut cargo_toml_value: toml::Value =
            toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

        let deps = cargo_toml_value
            .get_mut("dependencies")
            .expect("Missing dependencies section");
        if let toml::Value::Table(deps_table) = deps {
            if let toml::Value::Table(new_deps) = dependencies {
                for (key, value) in new_deps {
                    deps_table.insert(key.clone(), value.clone());
                }
            }
        }

        cargo_toml_content =
            toml::to_string(&cargo_toml_value).expect("Failed to serialize Cargo.toml");
        fs::write(cargo_toml_path, cargo_toml_content).expect("Failed to write Cargo.toml");
    }

    fn transform(&self, mut syntax_tree: syn::File) -> syn::File {
        let mut new_items = Vec::new();

        for import in self.env.import() {
            new_items.push(import);
        }

        // filter #[host] attribute functions and imports
        // let filtered_items = syntax_tree
        //     .items
        //     .into_iter()
        //     .filter(|item| match item {
        //         syn::Item::Fn(func) => !func.attrs.iter().any(|attr| attr.path().is_ident("host")),
        //         syn::Item::Use(use_item) => !use_item
        //             .attrs
        //             .iter()
        //             .any(|attr| attr.path().is_ident("host")),
        //         _ => true,
        //     })
        //     .collect::<Vec<_>>();

        let filtered_items = process_items_with_attribute(syntax_tree.items, "host", false, true);

        for item in &filtered_items {
            println!("Filtered item: {}", item.to_token_stream());
        }

        for item in filtered_items {
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

    fn generate_host_code(&self, input_path: &str, host_file_path: &str) {
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

        let host_template = self.env.get_host_template();

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

fn prepend_host_imports_to_syn_tree(input_path: &str, host_file_path: &str) {
    let source_code =
        fs::read_to_string(input_path).expect("Failed to read the input Rust source file");

    let syntax_tree = parse_file(&source_code).expect("Failed to parse the input Rust source file");

    let host_imports: Vec<Item> = syntax_tree
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Use(use_item) if has_host_annotation(&use_item.attrs) => {
                Some(Item::Use(use_item))
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

    // prepend without the #[host] attribute
    host_syntax_tree.items = host_imports
        .into_iter()
        .map(|item| {
            if let Item::Use(mut use_item) = item {
                use_item.attrs.retain(|attr| !attr.path().is_ident("host"));
                Item::Use(use_item)
            } else {
                item
            }
        })
        .chain(host_syntax_tree.items)
        .collect();

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
        while current_line < source_lines.len() && !source_lines[current_line].contains("env::read")
        {
            current_line += 1;
        }

        if let Stmt::Local(local) = stmt {
            // Check if the statement has an initializer expression
            if let Some(init) = &local.init {
                let expr = &init.expr;
                if is_env_read_call(expr) {
                    // Extract function name and arguments from the comment
                    if let Some((func_name, args)) =
                        extract_function_from_comment(source_lines[current_line])
                    {
                        // Generate the assignment line
                        if args.is_empty() {
                            assignments
                                .push(format!("let input{} = {}();", input_index, func_name));
                        } else {
                            assignments.push(format!(
                                "let input{} = {}({});",
                                input_index, func_name, args
                            ));
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
    println!(
        "Checking if expression is env::read() call: {}",
        expr.to_token_stream()
    );
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

// strip the given attribute from the syntax tree
fn strip_attribute(syntax_tree: &mut File, attribute_name: &str) {
    for item in &mut syntax_tree.items {
        match item {
            Item::Fn(func) => {
                func.attrs
                    .retain(|attr| !attr.path().is_ident(attribute_name));
            }
            Item::Use(use_item) => {
                use_item
                    .attrs
                    .retain(|attr| !attr.path().is_ident(attribute_name));
            }
            _ => {}
        }
    }
}

fn process_items_with_attribute(
    items: Vec<Item>,
    attribute_name: &str,
    strip_attribute: bool,
    strip_item: bool,
) -> Vec<Item> {
    items
        .into_iter()
        .filter_map(|item| {
            let mut keep_item = true;

            match &item {
                Item::Fn(func) => {
                    if func
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident(attribute_name))
                    {
                        if strip_item {
                            keep_item = false;
                        } else if strip_attribute {
                            let mut new_func = func.clone();
                            new_func
                                .attrs
                                .retain(|attr| !attr.path().is_ident(attribute_name));
                            return Some(Item::Fn(new_func));
                        }
                    }
                }
                Item::Use(use_item) => {
                    if use_item
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident(attribute_name))
                    {
                        if strip_item {
                            keep_item = false;
                        } else if strip_attribute {
                            let mut new_use_item = use_item.clone();
                            new_use_item
                                .attrs
                                .retain(|attr| !attr.path().is_ident(attribute_name));
                            return Some(Item::Use(new_use_item));
                        }
                    }
                }
                _ => {}
            }

            if keep_item {
                Some(item)
            } else {
                None
            }
        })
        .collect()
}
