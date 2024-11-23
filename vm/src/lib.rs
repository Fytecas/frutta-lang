use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use parser::statement::Statement;
use parser::expr::Expr;

pub struct VM {
    classes: Rc<RefCell<HashMap<String, Class>>>,
    variables: Rc<RefCell<HashMap<String, Value>>>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            classes: Rc::new(RefCell::new(HashMap::new())),
            variables: Rc::new(RefCell::new(HashMap::new())),
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
        self.classes.borrow_mut().insert("Number".to_string(), number_class);
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
                self.variables.borrow_mut().insert(name, value);
            }
            Statement::Assign(name, expr) => {
                let value = self.eval_expr(expr);
                if self.variables.borrow().contains_key(&name) {
                    self.variables.borrow_mut().insert(name, value);
                } else {
                    panic!("Variable {} not declared", name);
                }
            }
            Statement::Fn { name, params, body } => {
                let function = Function::UserDefined {
                    name: name.clone(),
                    params,
                    body,
                    classes: Rc::clone(&self.classes),
                };
                self.variables.borrow_mut().insert(name, Value::Function(function));
            }
            Statement::Expr(expr) => {
                let value = self.eval_expr(expr);
                
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
                self.variables.borrow().get(&name).cloned().unwrap_or_else(|| panic!("Variable {} not found", name))
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                let lhs = self.eval_expr(*lhs);
                let rhs = self.eval_expr(*rhs);
                self.eval_binary_op(op, lhs, rhs)
            }
            Expr::Call(function, args) => {
                let function = self.eval_expr(*function);
                if let Value::Function(function) = function {
                    let args = args.into_iter().map(|arg| self.eval_expr(arg)).collect();
                    function.call(args, Rc::clone(&self.variables))
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
                let classes = self.classes.borrow();
                let class = classes.get("Number").expect("Class 'Number' not found");
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
                let method = class.methods.get(method_name).expect(&format!("Method '{}' not found in class 'Number'", method_name));
                method.call(vec![Value::Number(lhs), Value::Number(rhs)], Rc::clone(&self.variables))
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
        classes: Rc<RefCell<HashMap<String, Class>>>,
    },
}

impl Function {
    pub fn call(&self, args: Vec<Value>, variables: Rc<RefCell<HashMap<String, Value>>>) -> Value {
        match self {
            Function::Builtin(func) => func(args),
            Function::UserDefined { name, params, body, classes } => {
                let mut local_variables = variables.borrow().clone();
                for (param, arg) in params.iter().zip(args.iter()) {
                    local_variables.insert(param.clone(), arg.clone());
                }
                // Insert the function into the variables so it can be called recursively
                local_variables.insert(name.to_string(), Value::Function(self.clone()));
                let mut vm = VM {
                    classes: Rc::clone(classes),
                    variables: Rc::new(RefCell::new(local_variables)),
                };
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

