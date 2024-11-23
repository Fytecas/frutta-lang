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
        let classes = Rc::new(RefCell::new(HashMap::new()));
        VM::init_builtin_classes(&classes);

        let vm = VM {
            classes: classes,
            variables: Rc::new(RefCell::new(HashMap::new())),
        };
        vm
    }

    fn init_builtin_classes(classes: &Rc<RefCell<HashMap<String, Class>>>) {
        let number_class = Class {
            name: "Number".to_string(),
            methods: {
                let mut methods = HashMap::new();
                methods.insert(MagicMethod::Add, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs + rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert(MagicMethod::Sub, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs - rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert(MagicMethod::Mul, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs * rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert(MagicMethod::Div, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Number(lhs / rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert(MagicMethod::Equal, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs == rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert(MagicMethod::NotEqual, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs != rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert(MagicMethod::GreaterThan, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs > rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods.insert(MagicMethod::LessThan, Function::Builtin(|args| {
                    if let (Value::Number(lhs), Value::Number(rhs)) = (&args[0], &args[1]) {
                        Value::Boolean(lhs < rhs)
                    } else {
                        unimplemented!()
                    }
                }));
                methods
            },
        };
        classes.borrow_mut().insert("Number".to_string(), number_class);
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
                    function.call(args, Rc::clone(&self.variables), Rc::clone(&self.classes))
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
                    parser::tokens::Token::Plus => MagicMethod::Add,
                    parser::tokens::Token::Minus => MagicMethod::Sub,
                    parser::tokens::Token::Star => MagicMethod::Mul,
                    parser::tokens::Token::Divider => MagicMethod::Div,
                    parser::tokens::Token::Equal => MagicMethod::Equal,
                    parser::tokens::Token::NotEqual => MagicMethod::NotEqual,
                    parser::tokens::Token::GreaterThan => MagicMethod::GreaterThan,
                    parser::tokens::Token::LessThan => MagicMethod::LessThan,
                    _ => unimplemented!(),
                };
                let method = class.methods.get(&method_name).expect(&format!("Method '{:?}' not found in class 'Number'", method_name));
                method.call(vec![Value::Number(lhs), Value::Number(rhs)], Rc::clone(&self.variables), Rc::clone(&self.classes))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MagicMethod {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
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
    methods: HashMap<MagicMethod, Function>,
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
    pub fn call(&self, args: Vec<Value>, variables: Rc<RefCell<HashMap<String, Value>>>, classes: Rc<RefCell<HashMap<String, Class>>>) -> Value {
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

