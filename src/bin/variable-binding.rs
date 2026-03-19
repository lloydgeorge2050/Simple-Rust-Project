fn main() {
    // The let keyword
    let a = 42;
    println!("Let Keyworkd: {}", a);

    // Type annotation
    let b: i32 = 42;
    println!("Type annotation: {}", b);

    // Uninitialize Variables
    let c;
    c = 42;
    println!("Uninitailize Variables: {}", c);

    // Throwing values away
    let _ = 42;

    // Shadowing bindings
    let d = 18;
    let d = d + 20;
    println!("Shadowing bindings: {}", d);
}