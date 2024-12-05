use std::collections::HashMap;

use quote::ToTokens;
use syn::{self, ExprCall};
use std::fmt;


pub trait CodeEnv {
    fn read(&self) -> syn::Stmt;
    fn commit(&self, var_name: &str) -> syn::Stmt;
    fn import(&self) -> Vec<syn::Item>;
    fn generate_host_env(&self, assignments: &[String]) -> String;
    fn get_host_template(&self) -> String;
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

    fn generate_host_env(&self, assignments: &[String]) -> String {
        unimplemented!("Please choose the appropriate environment");
    }

    fn get_host_template(&self) -> String {
        unimplemented!("Please choose the appropriate environment");
    }
}

pub struct Sp1Env;

impl CodeEnv for Sp1Env {
    fn read(&self) -> syn::Stmt {
        let code = format!("sp1_zkvm::io::read();",);
        syn::parse_str(&code).unwrap()
    }

    fn commit(&self, var_name: &str) -> syn::Stmt {
        let code = format!("sp1_zkvm::io::commit(&{0});", var_name);
        syn::parse_str(&code).unwrap()
    }

    fn import(&self) -> Vec<syn::Item> {
        vec![
            syn::Item::Verbatim(syn::parse_str("#![no_main]").unwrap()),
            syn::Item::Macro(syn::parse_str("sp1_zkvm::entrypoint!(main);").unwrap()),
        ]
    }

    fn generate_host_env(&self, assignments: &[String]) -> String {
        let mut builder_code = String::from("let mut stdin = SP1Stdin::new();\n");
        for assignment in assignments {
            let var_name = assignment.split_whitespace().nth(1).unwrap_or("");
            builder_code.push_str(&format!("stdin.write(&{});\n", var_name));
        }
        builder_code
    }

    fn get_host_template(&self) -> String {
        String::from(include_str!("../host_templates/sp1.rs"))
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
        vec![syn::Item::Use(
            syn::parse_str("use risc0_zkvm::guest::env;").unwrap(),
        )]
    }

    fn generate_host_env(&self, assignments: &[String]) -> String {
        let mut builder_code = String::from("ExecutorEnv::builder()\n");
        for assignment in assignments {
            let var_name = assignment.split_whitespace().nth(1).unwrap_or("");
            builder_code.push_str(&format!("    .write(&{})\n", var_name));
        }
        builder_code.push_str("    .build()\n    .unwrap()");
        builder_code
    }

    fn get_host_template(&self) -> String {
        String::from(include_str!("../host_templates/risc_zero.rs"))
    }

}
