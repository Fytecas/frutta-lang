use std::time::Instant;

fn main() {
    const INPUT: &str = r#"
    fn fib(n) {
        return n+9
    }

    fib(10)
    "#;
    let start = Instant::now();
    let expr = parser::Parser::parse(INPUT);
    println!("{:#?}", expr);
    println!("Time for parsing: {:?}", start.elapsed());
    let start = Instant::now();
    let mut vm = vm::VM::new();
    if let Ok(expr) = expr {
        vm.exec_statement(expr);
        println!("Time for execution: {:?}", start.elapsed());
    }
}
