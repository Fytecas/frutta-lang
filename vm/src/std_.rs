use std::io::Write;

use crate::{Class, Value};

pub fn std_class() -> Class {
    let mut class = Class::new("Std");

    class.add_method("print", |args| {
        for arg in args {
            print!("{}", arg.borrow());
        }
        println!();
        Value::None
    });

    class.add_method("input", |args| {
        let mut input = String::new();
        for arg in args {
            print!("{:}", arg.borrow());
        }
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        Value::String(input.trim().to_string())
    });

    class
}

pub fn time_class() -> Class {
    let mut class = Class::new("Time");

    class.add_method("now", |_| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        Value::Number(now.as_secs_f64())
    });

    class.add_method("sleep", |args| {
        let seconds = args[0].borrow().as_number().unwrap();
        std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
        Value::None
    });

    class
}
