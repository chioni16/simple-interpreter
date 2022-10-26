use crate::lexer::Lexer;

fn runner(input: String) {
    let l = Lexer::from_string(input);

}
#[test]
fn lexer() {
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
}