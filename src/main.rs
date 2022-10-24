use interpreter::lexer::Lexer;

fn main() {
    println!("Hello, world!");
    let l = Lexer::from_string("\n\t  +(=adas02)045fn(let".into());
    for i in l {
        println!("{:?}", i);
    }
}
