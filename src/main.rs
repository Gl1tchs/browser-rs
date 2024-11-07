use html::parser::Parser;

fn main() {
    let input = include_str!("../assets/test.html");
    let parser = Parser::new(input);
    println!("{:?}", parser.parse());
}
