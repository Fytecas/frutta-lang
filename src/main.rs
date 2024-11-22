use std::time::Instant;

fn main() {
    const INPUT: &str = r#"
    fn fib(n) {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        return fib(n - 1) + fib(n - 2);
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
