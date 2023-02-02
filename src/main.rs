#![allow(dead_code, unused_macros)]

mod default_environment;
mod interpreter;
mod model;
mod parser;
mod utils;
#[macro_use]
mod macros;

use std::{sync::Arc, sync::Mutex};

use rust_lisp::{default_env, interpreter::eval_block, parser::parse, start_repl};

// 🦀 Try finding me! I'm hidden all around the code!

fn main() {
    match std::env::args().nth(1) {
        Some(code) => {
            let env_rc = Arc::new(Mutex::new(default_env()));

            println!(
                "{}",
                eval_block(env_rc, parse(&code).filter_map(|a| a.ok())).unwrap()
            );
        }
        None => start_repl(None),
    }
}
