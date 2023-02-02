use std::sync::Mutex;
use std::fmt::Debug;
use std::fmt::Display;
use std::iter::FromIterator;
use std::ptr;
use std::sync::Arc;

use super::{RuntimeError, Value};

/**
 * A Lisp list, implemented as a linked-list
 */
#[derive(Debug, Clone)]
pub struct List {
    head: Option<Arc<Mutex<ConsCell>>>,
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.head
            .as_ref()
            .zip(other.head.as_ref())
            .map(|(a, b)| {
                a.lock().unwrap().car == b.lock().unwrap().car
            })
            .unwrap_or(self.head.is_none() && other.head.is_none())
    }
}

impl Eq for List {}

impl List {
    pub const NIL: List = List { head: None };

    pub fn car(&self) -> Result<Value, RuntimeError> {
        self.head
            .as_ref()
            .map(|rc| rc.lock().unwrap().car.clone())
            .ok_or_else(|| RuntimeError {
                msg: String::from("Attempted to apply car on nil"),
            })
    }
    #[must_use]
    pub fn cdr(&self) -> List {
        List {
            head: self
                .head
                .as_ref()
                .and_then(|rc| rc.lock().unwrap().cdr.clone()),
        }
    }

    #[must_use]
    pub fn cons(&self, val: Value) -> List {
        List {
            head: Some(Arc::new(Mutex::new(ConsCell {
                car: val,
                cdr: self.head.clone(),
            }))),
        }
    }
}

impl<'a> List {
    pub fn into_iter(list: &'a List) -> ConsIterator {
        ConsIterator(list.head.clone())
    }
}

/// A `ConsCell` is effectively a linked-list node, where the value in each node
/// is a lisp `Value`. To be used as a true "list", the ConsCell must be wrapped
/// in Value::List().
#[derive(Debug)]
struct ConsCell {
    pub car: Value,
    pub cdr: Option<Arc<Mutex<ConsCell>>>,
}

impl std::hash::Hash for ConsCell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.car.hash(state);
        if let Some(cdr) = self.cdr.as_ref() {
            ptr::hash(cdr, state)
        } else {
            None::<ConsCell>.hash(state)
        }
    }
}

impl PartialEq for ConsCell {
    fn eq(&self, other: &Self) -> bool {
        self.car == other.car
    }
}

impl Eq for ConsCell {}

impl Display for List {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(head) = &self.head {
            write!(formatter, "({})", head.lock().unwrap())
        } else {
            write!(formatter, "NIL")
        }
    }
}

impl Display for ConsCell {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.cdr.as_ref() {
            Some(cdr) => write!(formatter, "{} {}", self.car, cdr.lock().unwrap()),
            None => write!(formatter, "{}", self.car),
        }
    }
}

impl std::hash::Hash for List {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(head) = self.head.as_ref() {
            ptr::hash(head, state)
        } else {
            None::<ConsCell>.hash(state)
        }
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = Value;
    type IntoIter = ConsIterator;

    fn into_iter(self) -> Self::IntoIter {
        ConsIterator(self.head.clone())
    }
}

#[derive(Clone)]
pub struct ConsIterator(Option<Arc<Mutex<ConsCell>>>);

impl Iterator for ConsIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.clone().map(|cons| {
            let cons = cons.lock().unwrap();

            let val = cons.car.clone();
            self.0 = cons.cdr.clone();

            val
        })
    }
}

impl ExactSizeIterator for ConsIterator {
    fn len(&self) -> usize {
        let mut length: usize = 0;

        self.clone().for_each(|_| length += 1);

        length
    }
}

impl FromIterator<Value> for List {
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        let mut new_list = List { head: None };
        let mut tail: Option<Arc<Mutex<ConsCell>>> = None;

        for val in iter {
            // The cons cell for the current value
            let new_cons = Arc::new(Mutex::new(ConsCell {
                car: val,
                cdr: None,
            }));

            // if this is the first cell, put it in the List
            if new_list.head.is_none() {
                new_list.head = Some(new_cons.clone());
            // otherwise, put it in the current tail cell
            } else if let Some(tail_cons) = tail {
                tail_cons.as_ref().lock().unwrap().cdr = Some(new_cons.clone());
            }

            // the current cell is the new tail
            tail = Some(new_cons);
        }

        new_list
    }
}

impl<'a> FromIterator<&'a Value> for List {
    fn from_iter<I: IntoIterator<Item = &'a Value>>(iter: I) -> Self {
        iter.into_iter().cloned().collect()
    }
}
