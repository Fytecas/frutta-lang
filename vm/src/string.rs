use std::{cell::RefCell, rc::Rc};

use crate::{
    class::{Class, ClassInstance},
    MagicMethod, Value,
};

#[derive(Debug)]
pub struct StringClass;

impl Class for StringClass {
    fn create_instance(&self) -> Rc<dyn ClassInstance> {
        Rc::new(StringInstance {
            value: String::new(),
        })
    }
}

pub struct StringInstance {
    pub value: String,
}

impl std::fmt::Debug for StringInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.value)
    }
}

impl ClassInstance for StringInstance {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_field(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        if name == "value" {
            Some(Rc::new(RefCell::new(Value::ClassInstance(Rc::new(
                StringInstance {
                    value: self.value.clone(),
                },
            )))))
        } else {
            None
        }
    }

    fn call_magic(&self, method: MagicMethod, args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        let rhs = if let Value::ClassInstance(instance) = &*args[1].borrow() {
            if let Some(Value::ClassInstance(rhs_instance)) =
                instance.get_field("value").map(|v| v.borrow().clone())
            {
                if let Some(rhs_instance) = rhs_instance.as_any().downcast_ref::<StringInstance>() {
                    rhs_instance.value.clone()
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
            MagicMethod::Add => format!("{}{}", self.value, rhs),
            MagicMethod::Equal => return Rc::new(RefCell::new(Value::Boolean(self.value == rhs))),
            MagicMethod::NotEqual => {
                return Rc::new(RefCell::new(Value::Boolean(self.value != rhs)))
            }
            _ => unimplemented!(),
        };

        Rc::new(RefCell::new(Value::ClassInstance(Rc::new(
            StringInstance { value: result },
        ))))
    }
}
