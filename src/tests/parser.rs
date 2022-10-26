use crate::lexer::Lexer;
use crate::ast::statement::Program;
use crate::parser::{Parser, ParseResult};

fn runner(input: String) -> ParseResult<Program> {
    let l = Lexer::from_string(input);
    let mut p = Parser::new(l);
    p.parse_program()
}

#[test]
fn parser() {
    let input: String = "
        let five = -5;
        let ten = +10;
        let fifteen = ten - minus;
        let twentyFive = fifteen * 2 + five
        let OneZeroFive = (fifteen * (2 + five));
        let a = true;
        let b = false;
        let c = if 40 + 50 { true } else {80};
        let d = {
            let e = 42;
            e + 32
        };
        let f = fn(a, b, c) {
            let d = a+b-c;
            d * 2
        };
        let g = f(45,54,68,);
        let h = fn(b, c) {
            b + c
        }(41, 98);
    ".into();
    let ast = runner(input);
    assert!(ast.is_ok());
}