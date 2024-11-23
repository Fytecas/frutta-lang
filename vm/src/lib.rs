pub mod std_;

use parser::expr::Expr;
use parser::statement::Statement;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std_::{std_class, time_class};

pub struct VM {
    classes: Rc<RefCell<HashMap<String, Class>>>,
    variables: Rc<RefCell<HashMap<String, Rc<RefCell<Value>>>>>,
}

impl VM {
    pub fn new() -> Self {
        let classes = Rc::new(RefCell::new(HashMap::new()));
        VM::init_builtin_classes(&classes);
        let vm = VM {
            classes,
            variables: Rc::new(RefCell::new(HashMap::new())),
        };
        vm
    }

    fn init_builtin_classes(classes: &Rc<RefCell<HashMap<String, Class>>>) {
        let number_class = Class {
            name: "Number".to_string(),
            fields: {
                let mut fields = HashMap::new();
                fields.insert(
                    "value".to_string(),
                    Rc::new(RefCell::new(Value::Number(0.0))),
                );
                fields
            },
            magics: {
                let mut methods = HashMap::new();
                methods.insert(
                    MagicMethod::Add,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Number(lhs + rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods.insert(
                    MagicMethod::Sub,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Number(lhs - rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods.insert(
                    MagicMethod::Mul,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Number(lhs * rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods.insert(
                    MagicMethod::Div,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Number(lhs / rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods.insert(
                    MagicMethod::Equal,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Boolean(lhs == rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods.insert(
                    MagicMethod::NotEqual,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Boolean(lhs != rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods.insert(
                    MagicMethod::GreaterThan,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Boolean(lhs > rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods.insert(
                    MagicMethod::LessThan,
                    Function::Builtin(|args| {
                        if let (Value::Number(lhs), Value::Number(rhs)) =
                            (&*args[0].borrow(), &*args[1].borrow())
                        {
                            Value::Boolean(lhs < rhs)
                        } else {
                            unimplemented!()
                        }
                    }),
                );
                methods
            },
        };
        classes
            .borrow_mut()
            .insert("Number".to_string(), number_class);

        let std_class = std_class();
        classes
            .borrow_mut()
            .insert(std_class.name.clone(), std_class);

        classes
            .borrow_mut()
            .insert("Time".to_string(), time_class());
    }

    pub fn exec_statement(&mut self, stmt: Statement) -> Option<Rc<RefCell<Value>>> {
        match stmt {
            Statement::Block(statements) => {
                for statement in statements {
                    self.exec_statement(statement);
                }
            }
            Statement::Assign(name, expr) => {
                let value = self.eval_expr(expr);
                self.variables.borrow_mut().insert(name, value);
            }
            Statement::Fn { name, params, body } => {
                let function = Function::UserDefined {
                    name: name.clone(),
                    params,
                    body,
                    classes: Rc::clone(&self.classes),
                };
                self.variables
                    .borrow_mut()
                    .insert(name, Rc::new(RefCell::new(Value::Function(function))));
            }
            Statement::Expr(expr) => {
                let value = self.eval_expr(expr);
                // println!("{:?}", value.borrow());
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
                let condition_value = condition.borrow().clone();
                if let Value::Boolean(true) = condition_value {
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

    fn eval_expr(&self, expr: Expr) -> Rc<RefCell<Value>> {
        match expr {
            Expr::Number(n) => {
                // // Create a new Number class instance
                // let classes = self.classes.borrow();
                // let class = classes.get("Number").unwrap_or_else(|| panic!("Class 'Number' not found"));
                // let mut instance = ClassInstance {
                //     name: class.name.clone(),
                //     fields: class.fields.clone(),
                // };
                // // Set the value of the instance to the number
                // instance.fields.insert("value".to_string(), Rc::new(RefCell::new(Value::Number(n))));

                // Rc::new(RefCell::new(Value::ClassInstance(instance)))
                Rc::new(RefCell::new(Value::Number(n)))
            }
            Expr::Boolean(b) => Rc::new(RefCell::new(Value::Boolean(b))),
            Expr::Identifier(name) => {
                self.variables
                    .borrow()
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| {
                        let classes = self.classes.borrow();
                        let class = classes
                            .get(&name)
                            .unwrap_or_else(|| panic!("Variable or class '{}' not found", name));
                        Rc::new(RefCell::new(Value::ClassInstance(ClassInstance {
                            name: class.name.clone(),
                            fields: class.fields.clone(),
                        })))
                    })
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                let lhs = self.eval_expr(*lhs);
                let rhs = self.eval_expr(*rhs);
                self.eval_binary_op(op, lhs, rhs)
            }
            Expr::Acessor(accessors) => {
                let mut iter = accessors.into_iter();
                let origin = self.eval_expr(iter.next().unwrap());

                iter.fold(origin, |acc, accessor| match &*acc.borrow() {
                    Value::ClassInstance(instance) => {
                        let field_name = match accessor {
                            Expr::Identifier(name) => name,
                            _ => panic!("Invalid accessor expression"),
                        };
                        instance
                            .fields
                            .get(&field_name)
                            .cloned()
                            .unwrap_or_else(|| panic!("Field '{}' not found", field_name))
                    }
                    _ => panic!("Attempted to access a field on a non-class instance value"),
                })
            }
            Expr::Call(function, args) => {
                let function = self.eval_expr(*function);
                let args = args.into_iter().map(|arg| self.eval_expr(arg)).collect();
                let function_value = function.borrow();
                if let Value::Function(function) = &*function_value {
                    function.call(args, Rc::clone(&self.variables), Rc::clone(&self.classes))
                } else {
                    panic!("Attempted to call a non-function value");
                }
            }
            Expr::String(s) => Rc::new(RefCell::new(Value::String(s))),
            _ => unimplemented!(),
        }
    }

    fn eval_binary_op(
        &self,
        op: parser::tokens::Token,
        lhs: Rc<RefCell<Value>>,
        rhs: Rc<RefCell<Value>>,
    ) -> Rc<RefCell<Value>> {
        let magic = match op {
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
        let lhs_value = lhs.borrow().clone();
        let rhs_value = rhs.borrow().clone();

        match (&lhs_value, &rhs_value) {
            (Value::ClassInstance(class), Value::ClassInstance(_)) => {
                let classes = self.classes.borrow();
                let lhs_class = classes.get(&class.name).unwrap();
                let method = lhs_class.magics.get(&magic).unwrap();
                method.call(
                    vec![lhs, rhs],
                    Rc::clone(&self.variables),
                    Rc::clone(&self.classes),
                )
            }
            (Value::Number(lhs), Value::Number(rhs)) => match magic {
                MagicMethod::Add => Rc::new(RefCell::new(Value::Number(lhs + rhs))),
                MagicMethod::Sub => Rc::new(RefCell::new(Value::Number(lhs - rhs))),
                MagicMethod::Mul => Rc::new(RefCell::new(Value::Number(lhs * rhs))),
                MagicMethod::Div => Rc::new(RefCell::new(Value::Number(lhs / rhs))),
                MagicMethod::Equal => Rc::new(RefCell::new(Value::Boolean(lhs == rhs))),
                MagicMethod::NotEqual => Rc::new(RefCell::new(Value::Boolean(lhs != rhs))),
                MagicMethod::GreaterThan => Rc::new(RefCell::new(Value::Boolean(lhs > rhs))),
                MagicMethod::LessThan => Rc::new(RefCell::new(Value::Boolean(lhs < rhs))),
            },
            (Value::String(lhs), Value::String(rhs)) => match magic {
                MagicMethod::Add => Rc::new(RefCell::new(Value::String(format!("{}{}", lhs, rhs)))),
                MagicMethod::Equal => Rc::new(RefCell::new(Value::Boolean(lhs == rhs))),
                MagicMethod::NotEqual => Rc::new(RefCell::new(Value::Boolean(lhs != rhs))),
                _ => unimplemented!(),
            },
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
    None,
    Number(f64),
    String(String),
    Boolean(bool),
    ClassInstance(ClassInstance),
    Function(Function),
}

// as_number, as_string, as_boolean
impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "None"),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::ClassInstance(instance) => write!(f, "{:?}", instance),
            Value::Function(_) => write!(f, "<function>")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    magics: HashMap<MagicMethod, Function>,
    fields: HashMap<String, Rc<RefCell<Value>>>,
}

impl Class {
    pub fn new(name: &str) -> Self {
        Class {
            name: name.to_string(),
            magics: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    pub fn add_method(&mut self, name: &str, function: fn(Vec<Rc<RefCell<Value>>>) -> Value) {
        let method = Function::Builtin(function);
        let method = Rc::new(RefCell::new(Value::Function(method)));
        self.fields.insert(name.to_string(), method);
    }

    pub fn add_field<T, F>(&mut self, name: &str, value: T, f: F)
    where
        T: Into<Value>,
        F: Fn(&Value) -> bool,
    {
        let value = value.into();
        let value = Rc::new(RefCell::new(value));
        self.fields.insert(name.to_string(), value);
    }
}

#[derive(Debug, Clone)]
pub struct ClassInstance {
    name: String,
    fields: HashMap<String, Rc<RefCell<Value>>>,
}

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(fn(Vec<Rc<RefCell<Value>>>) -> Value),
    UserDefined {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        classes: Rc<RefCell<HashMap<String, Class>>>,
    },
}

impl Function {
    pub fn call(
        &self,
        args: Vec<Rc<RefCell<Value>>>,
        variables: Rc<RefCell<HashMap<String, Rc<RefCell<Value>>>>>,
        classes: Rc<RefCell<HashMap<String, Class>>>,
    ) -> Rc<RefCell<Value>> {
        match self {
            Function::Builtin(func) => Rc::new(RefCell::new(func(args))),
            Function::UserDefined {
                name,
                params,
                body,
                classes,
            } => {
                let mut local_variables = variables.borrow().clone();
                for (param, arg) in params.iter().zip(args.iter()) {
                    local_variables.insert(param.clone(), Rc::clone(arg));
                }
                // Insert the function into the variables so it can be called recursively
                local_variables.insert(
                    name.to_string(),
                    Rc::new(RefCell::new(Value::Function(self.clone()))),
                );
                let mut vm = VM {
                    classes: Rc::clone(classes),
                    variables: Rc::new(RefCell::new(local_variables)),
                };
                for statement in body {
                    if let Some(return_value) = vm.exec_statement(statement.clone()) {
                        return return_value;
                    }
                }
                Rc::new(RefCell::new(Value::None))
            }
        }
    }
}
