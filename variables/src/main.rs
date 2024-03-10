fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const Z : i32 = 4;
    println!("The value of z is: {}", Z);

    fn something() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value of x is: {}", x);
    
        let spaces = "  ";
        let spaces = spaces.len();
    }
}

// La variable main corre por su cuenta, no necesitas llamarla. 


fn floating() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
} 

fn pretty_math() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
}

fn booleanos() {
    let q = true;

    let w: bool = false; // with explicit type annotation
}

fn character() {
    let d = "f";
    let f = " ";
    let empty_cat_eyed = "   ";
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 5.4, 60);
}

fn more_tuple() {
    let g: (i32, f64, u8) = (500, 5.4, 60);
    let (a, b, c) = g;
    println!("the value of a is {}", a);
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
}
fn error_10_5() {
    let a = [1, 2, 3, 4, 5];
    let index = 5;
    let element = a[index];
    println!("The value of element is: {}", element);
    }