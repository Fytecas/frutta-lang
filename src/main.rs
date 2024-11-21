use std::time::Instant;

fn main() {
    const INPUT: &str = "let number = 42 print(number)";
    let start = Instant::now();
    let expr = parser::Parser::parse(INPUT);
    println!("{:#?}", expr);
    println!("Time: {:?}", start.elapsed());
}
