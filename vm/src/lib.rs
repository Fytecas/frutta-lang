use std::collections::HashMap;
use parser::statement::Statement;
use parser::expr::Expr;

pub struct VM {
    classes: HashMap<String, Class>,
    variables: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            classes: HashMap::new(),
            variables: HashMap::new(),
        };
        vm.init_builtin_classes();
        vm
    }

    fn init_builtin_classes(&mut self) {
        let number_class = Class {
            name: "Number".to_string(),
            methods: {
                let mut methods = HashMap::new();
                methods.insert("Add".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs + rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert("Sub".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs - rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert("Mul".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs * rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert("Div".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs / rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert("Equal".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs == rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert("NotEqual".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs != rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert("GreaterThan".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs > rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert("LessThan".to_string(), Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs < rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods
            },
        };
        self.classes.insert("Number".to_string(), number_class);
    }

    pub fn exec_statement(&mut self, stmt: Statement) -> Option<Value> {
        match stmt {
            Statement::Block(statements) => {
                for statement in statements {
                    self.exec_statement(statement);
                }
            }
            Statement::Let(name, expr) => {
                let value = self.eval_expr(expr);
                self.variables.insert(name, value);
            }
            Statement::Assign(name, expr) => {
                let value = self.eval_expr(expr);
                if self.variables.contains_key(&name) {
                    self.variables.insert(name, value);
                } else {
                    panic!("Variable {} not declared", name);
                }
            }
            Statement::Fn { name, params, body } => {
                let function = Function::UserDefined {
                    name: name.clone(),
                    params,
                    body,
                };
                self.variables.insert(name, Value::Function(function));
            }
            Statement::Expr(expr) => {
                let value = self.eval_expr(expr);
                println!("{:?}", value);
            }
            Statement::Return(expr) => {
                let value = self.eval_expr(expr);
                return Some(value);
            }
            Statement::If {
                condition,
                body,
                else_body,
            } => {
                let condition = self.eval_expr(condition);
                if let Value::Boolean(true) = condition {
                    for statement in body {
                        if let Some(return_value) = self.exec_statement(statement) {
                            return Some(return_value);
                        }
                    }
                } else {
                    for statement in else_body {
                        if let Some(return_value) = self.exec_statement(statement) {
                            return Some(return_value);
                        }
                    }
                }
            }
            _ => unimplemented!(),
        }
        None
    }

    fn eval_expr(&self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::Boolean(b) => Value::Boolean(b),
            Expr::Identifier(name) => {
                
                self.variables.get(&name).cloned().unwrap_or_else(|| panic!("Variable {} not found", name))
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                let lhs = self.eval_expr(*lhs);
                let rhs = self.eval_expr(*rhs);
                self.eval_binary_op(op, lhs, rhs)
            }
            Expr::Call ( function, args )=> {
                let function = self.eval_expr(*function);
                if let Value::Function(function) = function {
                    let args = args.into_iter().map(|arg| self.eval_expr(arg)).collect();
                    function.call(args)
                } else {
                    panic!("Attempted to call a non-function value");
                }
            }
            _ => unimplemented!(),
        }
    }

    fn eval_binary_op(&self, op: parser::tokens::Token, lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                let class = self.classes.get("Number").unwrap();
                let method_name = match op {
                    // TODO: implement the magic methods into a enum
                    parser::tokens::Token::Plus => "Add",
                    parser::tokens::Token::Minus => "Sub",
                    parser::tokens::Token::Star => "Mul",
                    parser::tokens::Token::Divider => "Div",
                    parser::tokens::Token::Equal => "Equal",
                    parser::tokens::Token::NotEqual => "NotEqual",
                    parser::tokens::Token::GreaterThan => "GreaterThan",
                    parser::tokens::Token::LessThan => "LessThan",
                    _ => unimplemented!(),
                };
                let method = class.methods.get(method_name).unwrap();
                method.call(vec![Value::Number(lhs), Value::Number(rhs)])
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    ClassInstance(ClassInstance),
    Function(Function),
}

#[derive(Debug)]
pub struct Class {
    name: String,
    methods: HashMap<String, Function>,
}

#[derive(Debug, Clone)]
pub struct ClassInstance {
    name: String,
    fields: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(fn(Vec<Value>) -> Value),
    UserDefined {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

impl Function {
    pub fn call(&self, args: Vec<Value>) -> Value {
        match self {
            Function::Builtin(func) => func(args),
            Function::UserDefined { name, params, body } => {
                let mut vm = VM::new();
                for (param, arg) in params.iter().zip(args.iter()) {
                    vm.variables.insert(param.clone(), arg.clone());
                }
                // Insert the function into the variables so it can be called recursively
                vm.variables.insert(name.to_string(), Value::Function(self.clone()));
                for statement in body {
                    if let Some(return_value) = vm.exec_statement(statement.clone()) {
                        return return_value;
                    }
                }
                Value::Number(0.0)
            }
        }
    }
}

