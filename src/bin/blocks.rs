fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    // Test block
    let a = "out";
    {
        // this is a different `x`
        let a = "in";
        println!("Block Inside: {}", a);
    }
    println!("Block Outside: {}", a);

    // Block as expresssion
    let b = { 42 };
    println!("Block Expression 1: {}", b);

    let c = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("Block Expression 2: {}", c);

    // Use return keyword
    println!("Return Keyworkd: {}", sum(c, b));

    // if expression
    let d = if b < c { b } else { c };
    println!("If Expression: {}", d);

    // match expression
    let e = match b > c { true => 6, false => 4 };
    println!("Match Expression: {}", e);
}