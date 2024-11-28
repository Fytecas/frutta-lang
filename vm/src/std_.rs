use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Utc};

use crate::{
    class::{Class, ClassInstance},
    number::NumberInstance,
    string::StringInstance,
    BuiltinFunction, Function, MagicMethod, Value,
};

#[derive(Debug)]
pub struct StdInstance;

impl ClassInstance for StdInstance {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_field(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        match name {
            "print" => Some(Rc::new(RefCell::new(Value::Function(Function::Builtin(
                BuiltinFunction::new(|args| {
                    for arg in args {
                        print!("{:}", arg.borrow());
                    }
                    println!();
                    Value::None
                }),
            ))))),
            "input" => Some(Rc::new(RefCell::new(Value::Function(Function::Builtin(
                BuiltinFunction::new(|args| {
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    Value::ClassInstance(Rc::new(StringInstance {
                        value: input.trim().to_string(),
                    }))
                }),
            ))))),
            "Time" => Some(Rc::new(RefCell::new(Value::ClassInstance(Rc::new(
                TimeInstance,
            ))))),
            _ => None,
        }
    }

    fn call_magic(&self, method: MagicMethod, args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct TimeClass;

impl Class for TimeClass {
    fn create_instance(&self) -> Rc<dyn ClassInstance> {
        Rc::new(TimeInstance)
    }
}

#[derive(Debug)]
pub struct TimeInstance;

impl ClassInstance for TimeInstance {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_field(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        match name {
            "now" => Some(Rc::new(RefCell::new(Value::Function(Function::Builtin(
                BuiltinFunction::new(|_| {
                    Value::ClassInstance(Rc::new(DatetimeInstance { value: Utc::now() }))
                }),
            ))))),
            "sleep" => Some(Rc::new(RefCell::new(Value::Function(Function::Builtin(
                BuiltinFunction::new(|args| {
                    let duration = args[0]
                        .borrow()
                        .as_any()
                        .downcast_ref::<NumberInstance>()
                        .expect("Expected number")
                        .value;
                    std::thread::sleep(std::time::Duration::from_secs_f64(duration));
                    Value::None
                }),
            ))))),
            _ => None,
        }
    }

    fn call_magic(&self, method: MagicMethod, args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        unimplemented!()
    }
}

// Datetime class
// Represents a date and time value
#[derive(Debug)]
pub struct DatetimeClass;

impl Class for DatetimeClass {
    fn create_instance(&self) -> Rc<dyn ClassInstance> {
        Rc::new(DatetimeInstance { value: Utc::now() })
    }
}

pub struct DatetimeInstance {
    pub value: DateTime<Utc>,
}

impl std::fmt::Debug for DatetimeInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl ClassInstance for DatetimeInstance {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_field(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        match name {
            "format" => {
                let datetime = self.value;
                Some(Rc::new(RefCell::new(Value::Function(Function::Builtin(
                    BuiltinFunction::new(move |args| {
                        let format = args[0]
                            .borrow()
                            .as_any()
                            .downcast_ref::<StringInstance>()
                            .expect("Expected string")
                            .value
                            .clone();
                        let datetime = datetime;
                        let formatted = datetime.format(&format).to_string();
                        Value::ClassInstance(Rc::new(StringInstance { value: formatted }))
                    }),
                )))))
            }
            _ => None,
        }
    }

    fn call_magic(&self, method: MagicMethod, args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        unimplemented!()
    }
}
