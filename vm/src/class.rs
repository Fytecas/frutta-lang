use std::{cell::RefCell, rc::Rc};

use crate::{MagicMethod, Value};

pub trait Class: std::fmt::Debug {
    fn create_instance(&self) -> Rc<dyn ClassInstance>;
}

pub trait ClassInstance: std::fmt::Debug {
    fn get_field(&self, name: &str) -> Option<Rc<RefCell<Value>>>;
    fn call_magic(&self, method: MagicMethod, args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>>;
    fn as_any(&self) -> &dyn std::any::Any;
}
