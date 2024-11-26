use std::{cell::RefCell, rc::Rc};

use crate::{
    class::{Class, ClassInstance},
    MagicMethod, Value,
};

#[derive(Debug)]
pub struct NumberClass;

impl Class for NumberClass {
    fn create_instance(&self) -> Rc<dyn ClassInstance> {
        Rc::new(NumberInstance { value: 0.0 })
    }
}

pub struct NumberInstance {
    pub value: f64,
}

impl NumberInstance {
    pub fn new(value: f64) -> Self {
        NumberInstance { value }
    }
}

impl std::fmt::Debug for NumberInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.value)
    }
}

impl ClassInstance for NumberInstance {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_field(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        if name == "value" {
            Some(Rc::new(RefCell::new(Value::ClassInstance(Rc::new(
                NumberInstance { value: self.value },
            )))))
        } else {
            match name {
                "abs" => {
                    let value = self.value;
                    Some(Rc::new(RefCell::new(Value::Function(crate::Function::Builtin(crate::BuiltinFunction::new(
                        move |_args| {
                            return Value::ClassInstance(Rc::new(NumberInstance::new(value.abs())));
                        }
                    ))))))
                },
                _ => None,
            }
        }
    }

    fn call_magic(&self, method: MagicMethod, args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        let rhs = if let Value::ClassInstance(instance) = &*args[1].borrow() {
            if let Some(Value::ClassInstance(rhs_instance)) =
                instance.get_field("value").map(|v| v.borrow().clone())
            {
                if let Some(rhs_instance) = rhs_instance.as_any().downcast_ref::<NumberInstance>() {
                    rhs_instance.value
                } else {
                    panic!("Invalid type for rhs")
                }
            } else {
                panic!("Invalid type for rhs")
            }
        } else {
            panic!("Invalid type for rhs")
        };

        let result = match method {
            MagicMethod::Add => self.value + rhs,
            MagicMethod::Sub => self.value - rhs,
            MagicMethod::Mul => self.value * rhs,
            MagicMethod::Div => self.value / rhs,
            MagicMethod::Equal => return Rc::new(RefCell::new(Value::Boolean(self.value == rhs))),
            MagicMethod::NotEqual => {
                return Rc::new(RefCell::new(Value::Boolean(self.value != rhs)))
            }
            MagicMethod::GreaterThan => {
                return Rc::new(RefCell::new(Value::Boolean(self.value > rhs)))
            }
            MagicMethod::LessThan => {
                return Rc::new(RefCell::new(Value::Boolean(self.value < rhs)))
            }
        };

        Rc::new(RefCell::new(Value::ClassInstance(Rc::new(
            NumberInstance { value: result },
        ))))
    }
}
