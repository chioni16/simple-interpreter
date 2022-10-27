use interpreter::lexer::Lexer;
use interpreter::parser::Parser;
use interpreter::evaluation::eval_program;
fn main() {
    // let l = Lexer::from_string(
    //     "
    //     let five = 5;
    //     let ten = 10;
    //     let add = fn(x, y) {
    //     x + y;
    //     };
    //     let result = add(five, ten);
    //     !-/*5;
    //     5 < 10 > 5;
    //     if (5 < 10) {
    //     return true;
    //     } else {
    //     return false;
    //     }
    //     10 == 10;
    //     10 != 9;
    //     let five = 5;
    //     let ten = 10;
    //     let add = fn(x, y) {
    //     x + y;
    //     };
    //     let result = add(five, ten);
    //     !-/*5;
    //     5 < 10 > 5;
    // "
    //     .into(),
    // );
    // for i in l {
    //     println!("{:?}", i);
    // }



    // let l = Lexer::from_string(
    //     "
    //     let five = -5;
    //     let ten = +10;
    //     let fifteen = ten - minus;
    //     let twentyFive = fifteen * 2 + five
    //     let OneZeroFive = (fifteen * (2 + five));
    //     let a = true;
    //     let b = false;
    //     let c = if 40 + 50 { true } else {80};
    //     let d = {
    //         let e = 42;
    //         e + 32
    //     };
    //     let f = fn(a, b, c) {
    //         let d = a+b-c;
    //         d * 2
    //     };
    //     let g = f(45,54,68,);
    //     let h = fn(b, c) {
    //         b + c
    //     }(41, 98);
    // "
    //     .into(),
    // );
    // let mut p = Parser::new(l);
    // let ast = p.parse_program().unwrap();
    // println!("{:?}", ast);


    // let l = Lexer::from_string(r#"return 72 * 43; -5478/7; "#.into());
    // let l = Lexer::from_string(r#"1 * 2; -4; "#.into());
    // let l = Lexer::from_string(r#"5; true;   43 -5478/7+true; 3 + 4 * 5 == 3 * 1 + 4 * 5 if 5 > 3 {89+41} else { 98 -87}"#.into());
    // let l = Lexer::from_string(r#"
    //     if 6 > 3 {
    //         if 5> 3 {
    //             return 1
    //         } else {
    //             return 2
    //         }
    //     } else {
    //         return 3
    //     }
    // "#.into());
    let l = Lexer::from_string(r#"
        let x = 4;
        if (x > 1) {
            if (x > 5) {
                45;
            }
        } 
        let d = {
             let e = 42;
             e + 32
         };{
            5 + 20
         }
         let f = fn(a, b, c) {
             let d = a+b-c;
             d * 2
         };
         fn(a, b, c) {
             let d = a+b-c;
             d * 2
         }
         
    "#.into());
    let mut p = Parser::new(l);
    let ast = p.parse_program().unwrap();
    println!("{:?}", ast);
    let object = eval_program(ast);
    println!("{:?}", object);
}
