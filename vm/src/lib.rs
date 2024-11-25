mod class;
mod number;
mod std_;
mod string;

use crate::class::{Class, ClassInstance};
use crate::number::{NumberClass, NumberInstance};
use crate::string::{StringClass, StringInstance};
use parser::expr::Expr;
use parser::statement::Statement;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std_::{StdInstance, TimeClass};

pub struct VM {
    classes: Rc<RefCell<HashMap<String, Rc<dyn Class>>>>,
    variables: Rc<RefCell<HashMap<String, Rc<RefCell<Value>>>>>,
}

impl VM {
    pub fn new() -> Self {
        let classes = Rc::new(RefCell::new(HashMap::new()));
        VM::init_builtin_classes(&classes);
        VM {
            classes,
            variables: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn init_builtin_classes(classes: &Rc<RefCell<HashMap<String, Rc<dyn Class>>>>) {
        classes
            .borrow_mut()
            .insert("Number".to_string(), Rc::new(NumberClass));
        classes
            .borrow_mut()
            .insert("String".to_string(), Rc::new(StringClass));
        classes
            .borrow_mut()
            .insert("Std".to_string(), Rc::new(StdClass));
        classes
            .borrow_mut()
            .insert("Time".to_string(), Rc::new(TimeClass));
    }

    pub fn exec_statement(&mut self, stmt: &Statement) -> Option<Rc<RefCell<Value>>> {
        match stmt {
            Statement::Block(statements) => {
                for statement in statements {
                    if let Some(return_value) = self.exec_statement(statement) {
                        return Some(return_value);
                    }
                }
            }
            Statement::Assign(name, expr) => {
                let value = self.eval_expr(expr);
                self.variables.borrow_mut().insert(name.clone(), value);
            }
            Statement::Fn { name, params, body } => {
                let function = Function::UserDefined {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    classes: Rc::clone(&self.classes),
                };
                self.variables.borrow_mut().insert(
                    name.clone(),
                    Rc::new(RefCell::new(Value::Function(function))),
                );
            }
            Statement::Expr(expr) => {
                let _ = self.eval_expr(expr);
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
        }
        None
    }

    fn eval_expr(&self, expr: &Expr) -> Rc<RefCell<Value>> {
        match expr {
            Expr::Number(n) => Rc::new(RefCell::new(Value::ClassInstance(Rc::new(
                NumberInstance::new(*n),
            )))),
            Expr::Boolean(b) => Rc::new(RefCell::new(Value::Boolean(*b))),
            Expr::String(s) => Rc::new(RefCell::new(Value::ClassInstance(Rc::new(
                StringInstance { value: s.clone() },
            )))),
            Expr::Identifier(name) => {
                self.variables
                    .borrow()
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| {
                        let classes = self.classes.borrow();
                        let class = classes
                            .get(name)
                            .unwrap_or_else(|| panic!("Variable or class '{}' not found", name));
                        Rc::new(RefCell::new(Value::ClassInstance(class.create_instance())))
                    })
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                let lhs = self.eval_expr(lhs);
                let rhs = self.eval_expr(rhs);
                // TODO: Remove clone
                self.eval_binary_op(op.clone(), lhs, rhs)
            }
            Expr::Acessor(accessors) => {
                let mut iter = accessors.iter();
                let origin = self.eval_expr(iter.next().unwrap());

                iter.fold(origin, |acc, accessor| match &*acc.borrow() {
                    Value::ClassInstance(instance) => {
                        let field_name = match accessor {
                            Expr::Identifier(name) => name.clone(),
                            _ => panic!("Invalid accessor expression"),
                        };
                        instance
                            .get_field(&field_name)
                            .unwrap_or_else(|| panic!("Field '{}' not found", field_name))
                    }
                    _ => panic!("Attempted to access a field on a non-class instance value"),
                })
            }
            Expr::Call(function, args) => {
                let function = self.eval_expr(function);
                let args = args.iter().map(|arg| self.eval_expr(arg)).collect();
                let function_value = function.borrow();
                if let Value::Function(function) = &*function_value {
                    function.call(args, Rc::clone(&self.variables))
                } else {
                    panic!("Attempted to call a non-function value");
                }
            }
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
            (Value::ClassInstance(lhs_instance), Value::ClassInstance(_)) => {
                lhs_instance.call_magic(magic, vec![lhs, rhs])
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
    None,
    Boolean(bool),
    ClassInstance(Rc<dyn ClassInstance>),
    Function(Function),
}

impl Value {
    fn as_any(&self) -> &dyn std::any::Any {
        match self {
            Value::ClassInstance(instance) => instance.as_any(),
            _ => panic!("as_any is not implemented for this Value variant"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "None"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::ClassInstance(instance) => write!(f, "{:?}", instance),
            Value::Function(_) => write!(f, "<function>"),
        }
    }
}

#[derive(Debug)]
pub struct StdClass;

impl Class for StdClass {
    fn create_instance(&self) -> Rc<dyn ClassInstance> {
        Rc::new(StdInstance)
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(fn(Vec<Rc<RefCell<Value>>>) -> Value),
    UserDefined {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        classes: Rc<RefCell<HashMap<String, Rc<dyn Class>>>>,
    },
}

impl Function {
    pub fn call(
        &self,
        args: Vec<Rc<RefCell<Value>>>,
        variables: Rc<RefCell<HashMap<String, Rc<RefCell<Value>>>>>,
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
                local_variables.insert(
                    name.to_string(),
                    Rc::new(RefCell::new(Value::Function(self.clone()))),
                );
                let mut vm = VM {
                    classes: Rc::clone(classes),
                    variables: Rc::new(RefCell::new(local_variables)),
                };
                for statement in body {
                    if let Some(return_value) = vm.exec_statement(statement) {
                        return return_value;
                    }
                }
                Rc::new(RefCell::new(Value::None))
            }
        }
    }
}
