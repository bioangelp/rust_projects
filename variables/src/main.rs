fn main() {
    constables_and_variables();
    another_another_function(5);
    another_function();
    something();
    floating();
    pretty_math();
    booleanos();
    character();
    tuples();
    more_tuple();
    arrays();
    error_10_5();
}

fn constables_and_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const Z: i32 = 4;
    println!("The value of z is: {}", Z);
    
}

fn another_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn something() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "  ";
    let _spaces = spaces.len();
}

fn floating() {
    let k = 2.0; // f64
    let l: f32 = 3.0; // f32
    println!("The value of k is: {}", k);
    println!("The value of l is: {}", l);
}

fn pretty_math() {
    // addition
    let sum = 5 + 10;
    println!("The value 5 + 10 is {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of 95.5 - 4.3 is: {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("The value of 4 * 30 is: {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("The value of 56.7 / 32.2 is: {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("The value of 43 % 5 is: {}", remainder);
}

fn booleanos() {
    let q = true;
    println!("{}", q);
    let w: bool = false; // with explicit type annotation
    println!("{}", w);
}

fn character() {
    let d = "f";
    let f = " ";
    let empty_cat_eyed = "   ";
    println!("{}", d);
    println!("{}", f);
    println!("{}", empty_cat_eyed);
}

fn tuples() {
    let tup: (i32, f64, u8) = (10, 3.14, 5);
    println!("Tuple: {:?}", tup);
    // a trait represents a set of methods that can be used to extend the functionality of a class.
    // {:?} is use format a value using its debug representation. 
    // https://doc.rust-lang.org/std/fmt/trait.Debug.html
}

fn more_tuple() {
    let op: (i32, f64, u8) = (500, 5.4, 60);
    let (a, _, _) = op;
    println!("the value of a is {:?}", a);
}

fn arrays() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November","December"];
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let x = (5, 6, 7);
    let five = x.0;
    let six = x.1;
    let seven = x.2;
    println!("first: {:?}", arr);
    println!("first: {:?}", months);
    println!("first: {}", first);
    println!("second: {}", second);
    println!("five: {}", five);
    println!("six: {}", six);
    println!("seven: {}", seven);
}

fn error_10_5() {
    let a = [1, 2, 3, 4, 5];
    let index = 2;
    // Added check for array index out of bounds
    if index < a.len() {
        let element = a[index];
        println!("The value of element is: {}", element);
    } else {
        println!("Index out of bounds!");
    }
}

// La variable main corre por su cuenta, no necesitas llamarla. 