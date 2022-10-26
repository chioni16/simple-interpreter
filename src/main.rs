use interpreter::lexer::Lexer;
use interpreter::parser::Parser;

fn main() {
    let l = Lexer::from_string(
        "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
        return true;
        } else {
        return false;
        }
        10 == 10;
        10 != 9;
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
    "
        .into(),
    );
    for i in l {
        println!("{:?}", i);
    }
    let l = Lexer::from_string(
        "
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
    "
        .into(),
    );
    let mut p = Parser::new(l);
    let ast = p.parse_program().unwrap();
    println!("{:?}", ast);
}
