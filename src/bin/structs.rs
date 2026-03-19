struct Vec2 {
    x: f64,
    y: f64,
}

fn main() {
    let v1 = Vec2 {x: 12.0, y: 24.0};
    println!("({}, {})", v1.x, v1.y);

    let v2 = Vec2 {x: 15.0, y: 23.0};
    let v3 = Vec2 {x: 9.0, ..v2};
    println!("({}, {})", v2.x, v2.y);
    println!("({}, {})", v3.x, v3.y);

    let Vec2 {x, y} = v1;
    println!("({}, {})", x, y);
}