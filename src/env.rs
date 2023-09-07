use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Term;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Term>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Term> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, term: Term) {
        self.vars.insert(name.to_string(), term);
    }
}
