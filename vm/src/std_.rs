use crate::{Class, Value};

pub fn std_class() -> Class {
    let mut class = Class::new("Std");

    class.add_method("print", |args| {
        for arg in args {
            print!("{:?}", arg.borrow());
        }
        println!();
        Value::None
    });

    class
}