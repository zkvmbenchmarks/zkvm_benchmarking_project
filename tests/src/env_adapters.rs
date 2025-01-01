use std::collections::HashMap;

use quote::ToTokens;
use std::fmt;
use syn::{self, ExprCall};

pub trait CodeEnv {
    fn read(&self) -> syn::Stmt;
    fn commit(&self, var_name: &str) -> syn::Stmt;
    fn import(&self) -> Vec<syn::Item>;
    fn generate_host_env(&self, assignments: &[String]) -> String;
    fn get_host_template(&self) -> String;
    fn get_file_copy_destination(&self) -> String;
    fn get_host_cargo_toml_path(&self) -> String;
    fn get_guest_cargo_toml_path(&self) -> String;
    fn get_available_patches(&self) -> HashMap<String, String>;
    fn get_guest_output_dir(&self) -> String;
    fn get_host_output_dir(&self) -> String;
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

    fn get_file_copy_destination(&self) -> String {
        unimplemented!("Please choose the appropriate environment");
    }

    fn get_host_cargo_toml_path(&self) -> String {
        unimplemented!("Please choose the appropriate environment");
    }

    fn get_guest_cargo_toml_path(&self) -> String {
        unimplemented!("Please choose the appropriate environment");
    }

    fn get_available_patches(&self) -> HashMap<String, String> {
        unimplemented!("Please choose the appropriate environment");
    }

    fn get_guest_output_dir(&self) -> String {
        unimplemented!("Please choose the appropriate environment");
    }

    fn get_host_output_dir(&self) -> String {
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

    fn get_file_copy_destination(&self) -> String {
        String::from("../sp1_benchmarks/sp1_project/script/src/bin")
    }

    fn get_host_cargo_toml_path(&self) -> String {
        String::from("../sp1_benchmarks/sp1_project/script/Cargo.toml")
    }

    fn get_guest_cargo_toml_path(&self) -> String {
        String::from("../sp1_benchmarks/sp1_project/program/Cargo.toml")
    }

    fn get_available_patches(&self) -> HashMap<String, String> {
        let mut patches = HashMap::new();
        patches.insert(
            "sha2".to_string(),
            r#"
                [patch.crates-io]
                sha2-v0-9-8 = { git = "https://github.com/sp1-patches/RustCrypto-hashes", package = "sha2", tag = "sha2-v0.9.8-patch-v1" }
                sha2-v0-10-6 = { git = "https://github.com/sp1-patches/RustCrypto-hashes", package = "sha2", tag = "sha2-v0.10.6-patch-v1" }
                sha2-v0-10-8 = { git = "https://github.com/sp1-patches/RustCrypto-hashes", package = "sha2", tag = "sha2-v0.10.8-patch-v1" }
                "#.to_string(),
        );
        // Add more patches as needed
        patches
    }

    fn get_guest_output_dir(&self) -> String {
        String::from("../sp1_benchmarks/sp1_project/program/src")
    }

    fn get_host_output_dir(&self) -> String {
        String::from("../sp1_benchmarks/sp1_project/script/src/bin")
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
            builder_code.push_str("    .unwrap()\n");
        }
        builder_code.push_str("    .build()\n    .unwrap()");
        builder_code
    }

    fn get_host_template(&self) -> String {
        String::from(include_str!("../host_templates/risc_zero.rs"))
    }

    fn get_file_copy_destination(&self) -> String {
        String::from("../risc0_benchmarks/test_project/host/src")
    }

    fn get_host_cargo_toml_path(&self) -> String {
        String::from("../risc0_benchmarks/test_project/host/Cargo.toml")
    }

    fn get_guest_cargo_toml_path(&self) -> String {
        String::from("../risc0_benchmarks/test_project/methods/guest/Cargo.toml")
    }

    fn get_available_patches(&self) -> HashMap<String, String> {
        let mut patches = HashMap::new();

        patches.insert(
            "sha2".to_string(),
            r#"
                [patch.crates-io]
                sha2-v0-10-8 = { git = "https://github.com/risc0/RustCrypto-hashes", package = "sha2", tag = "sha2-v0.10.8-risczero.0" }
                sha2-v0-10-7 = { git = "https://github.com/risc0/RustCrypto-hashes", package = "sha2", tag = "sha2-v0.10.7-risczero.0" }
                sha2-v0-10-6 = { git = "https://github.com/risc0/RustCrypto-hashes", package = "sha2", tag = "sha2-v0.10.6-risczero.0" }
                sha2-v0-9-9 = { git = "https://github.com/risc0/RustCrypto-hashes", package = "sha2", tag = "sha2-v0.9.9-risczero.0" }
                "#.to_string(),
        );

        patches.insert(
            "k256".to_string(),
            r#"
                [patch.crates-io]
                k256-v0-13-4 = { git = "https://github.com/risc0/RustCrypto-elliptic-curves", package = "k256", tag = "k256/v0.13.4-risczero.1" }
                k256-v0-13-3 = { git = "https://github.com/risc0/RustCrypto-elliptic-curves", package = "k256", tag = "k256/v0.13.3-risczero.1" }
                k256-v0-13-2 = { git = "https://github.com/risc0/RustCrypto-elliptic-curves", package = "k256", tag = "k256/v0.13.2-risczero.1" }
                k256-v0-13-1 = { git = "https://github.com/risc0/RustCrypto-elliptic-curves", package = "k256", tag = "k256/v0.13.1-risczero.1" }
                "#.to_string(),
        );

        patches.insert(
            "curve25519-dalek".to_string(),
            r#"
                [patch.crates-io]
                curve25519-dalek-v4-1-2 = { git = "https://github.com/risc0/ed25519-dalek", package = "curve25519-dalek", tag = "curve25519-4.1.2-risczero.0" }
                curve25519-dalek-v4-1-1 = { git = "https://github.com/risc0/ed25519-dalek", package = "curve25519-dalek", tag = "curve25519-4.1.1-risczero.0" }
                curve25519-dalek-v4-1-0 = { git = "https://github.com/risc0/ed25519-dalek", package = "curve25519-dalek", tag = "curve25519-4.1.0-risczero.0" }
                "#.to_string(),
        );

        patches.insert(
            "rsa".to_string(),
            r#"
                [patch.crates-io]
                rsa-v0-9-6 = { git = "https://github.com/risc0/RustCrypto-RSA", package = "rsa", tag = "v0.9.6-risczero.0" }
                "#.to_string(),
        );

        patches.insert(
            "crypto-bigint".to_string(),
            r#"
                [patch.crates-io]
                crypto-bigint-v0-5-5 = { git = "https://github.com/risc0/RustCrypto-crypto-bigint", package = "crypto-bigint", tag = "v0.5.5-risczero.0" }
                crypto-bigint-v0-5-4 = { git = "https://github.com/risc0/RustCrypto-crypto-bigint", package = "crypto-bigint", tag = "v0.5.4-risczero.0" }
                crypto-bigint-v0-5-3 = { git = "https://github.com/risc0/RustCrypto-crypto-bigint", package = "crypto-bigint", tag = "v0.5.3-risczero.0" }
                crypto-bigint-v0-5-2 = { git = "https://github.com/risc0/RustCrypto-crypto-bigint", package = "crypto-bigint", tag = "v0.5.2-risczero.0" }
                "#.to_string(),
        );

        patches
    }

    fn get_guest_output_dir(&self) -> String {
        String::from("../risc0_benchmarks/test_project/methods/guest/src")
    }

    fn get_host_output_dir(&self) -> String {
        String::from("../risc0_benchmarks/test_project/host/src")
    }

}
