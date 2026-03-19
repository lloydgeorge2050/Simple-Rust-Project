fn main() {
    // tuple
    let a = ('a', 18);
    println!("Tuple: {}, {}", a.0, a.1);

    // destructuring
    let (b, c) = ('a', 18);
    println!("Tuple: {}, {}", b, c);
}