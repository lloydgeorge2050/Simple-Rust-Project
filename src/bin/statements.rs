fn main() {
    // single line statements
    let a = 3;
    let b = 6;
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    // multi line statements
    let d = vec![1, 2, 3, 4, 5, 6, 7, 8].iter().map(|x| x + 3).fold(0, |x, y| x + y);
    println!("Result: {}", d);
}