fn main() {
    another_function(5, 6);
    let x = five(3, 5);
    println!("The value of x is: {}", x);
    let g = plus_one(5);
    println!("The value of x is: {}", x);
    
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five(q: i32, w: i32) -> i32 {
    let e : i32 = q + w + 3;
    e
}

fn plus_one(g: i32) -> i32 {
    g + 1
}