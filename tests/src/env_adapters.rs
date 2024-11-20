use syn;

pub trait CodeEnv {
    fn read(&self) -> syn::Stmt;
    fn commit(&self, var_name: &str) -> syn::Stmt;
    fn import(&self) -> Vec<syn::Item>;
}

pub struct NotImplementedEnv;

impl CodeEnv for NotImplementedEnv {
    fn read(&self) -> syn::Stmt {
        unimplemented!("Please choose the appropriate environment");
    }

    fn commit(&self, _var_name: &str) -> syn::Stmt {
        unimplemented!("Please choose the appropriate environment");
    }

    fn import(&self) -> Vec<syn::Item> {
        unimplemented!("Please choose the appropriate environment");
    }
}

pub struct Sp1Env;

impl CodeEnv for Sp1Env {
    fn read(&self) -> syn::Stmt {
        let code = format!(
            "sp1_zkvm::io::read();",
        );
        syn::parse_str(&code).unwrap()
    }

    fn commit(&self, var_name: &str) -> syn::Stmt {
        let code = format!("sp1_zkvm::io::commit_slice(&{0});", var_name);
        syn::parse_str(&code).unwrap()
    }

    fn import(&self) -> Vec<syn::Item> {
        vec![
            syn::Item::Verbatim(syn::parse_str("#![no_main]").unwrap()),
            syn::Item::Macro(syn::parse_str("sp1_zkvm::entrypoint!(main);").unwrap()),
        ]
    }
}

pub struct Risc0Env;

impl CodeEnv for Risc0Env {
    fn read(&self) -> syn::Stmt {
        let code = format!("env::read();");
        syn::parse_str(&code).unwrap()
    }

    fn commit(&self, var_name: &str) -> syn::Stmt {
        let code = format!("env::commit(&{0});", var_name);
        syn::parse_str(&code).unwrap()
    }

    fn import(&self) -> Vec<syn::Item> {
        vec![
            syn::Item::Use(syn::parse_str("use risc0_zkvm::guest::env;").unwrap()),
        ]
    }
}
