use std::sync::Mutex;
use std::fmt::Debug;
use std::ptr;
use std::sync::Arc;

use super::{Env, Symbol, Value};

/// A Lisp function defined in Lisp.
#[derive(Debug, Clone)]
pub struct Lambda {
    pub closure: Arc<Mutex<Env>>,
    pub argnames: Vec<Symbol>,
    pub body: Arc<Value>,
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.closure, &other.closure)
            && self.argnames == other.argnames
            && self.body == other.body
    }
}

impl std::hash::Hash for Lambda {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&self.closure, state);
        self.argnames.hash(state);
        self.body.hash(state);
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body_str = format!("{}", &self.body);

        return write!(
            f,
            "({}) {}",
            self.argnames
                .iter()
                .map(|sym| sym.0.as_str())
                .collect::<Vec<&str>>()
                .join(" "),
            &body_str[1..body_str.chars().count() - 1]
        );
    }
}
