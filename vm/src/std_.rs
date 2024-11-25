use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{
    class::{Class, ClassInstance},
    number::NumberInstance,
    string::StringInstance,
    Function, MagicMethod, Value,
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
                |args| {
                    for arg in args {
                        print!("{}", arg.borrow());
                    }
                    println!();
                    Value::None
                },
            ))))),
            "input" => Some(Rc::new(RefCell::new(Value::Function(Function::Builtin(
                |args| {
                    let mut input = String::new();
                    for arg in args {
                        print!("{:}", arg.borrow());
                    }
                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut input).unwrap();
                    Value::ClassInstance(Rc::new(StringInstance {
                        value: input.trim().to_string(),
                    }))
                },
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
                |_| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap();
                    Value::ClassInstance(Rc::new(NumberInstance {
                        value: now.as_secs_f64(),
                    }))
                },
            ))))),
            "sleep" => Some(Rc::new(RefCell::new(Value::Function(Function::Builtin(
                |args| {
                    let seconds = args[0]
                        .borrow()
                        .to_owned()
                        .as_any()
                        .downcast_ref::<NumberInstance>()
                        .unwrap()
                        .value;
                    std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
                    Value::None
                },
            ))))),
            _ => None,
        }
    }

    fn call_magic(&self, method: MagicMethod, args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        unimplemented!()
    }
}
