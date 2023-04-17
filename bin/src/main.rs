use frontend::parse;
use middleend::translate;

fn main() {
    let program = "
fn factorial(n) {
    if n == 1 {
        return 1;
    }
    return factorial(n - 1) * n;
}

factorial(5); // 120
    ";

    println!("{}", translate(parse(program).unwrap()).unwrap());
}
