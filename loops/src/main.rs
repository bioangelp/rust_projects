// execute a block of code over and over again forever or until you explicitly tell it to stop.
// control = c, ^C or break



fn main() {

    let mut number = 3; // never forget mut to make mutable

    while number != 0 {
        println!("{}!", number);
        number = number -1;
    };

    println!("LIFTOFF!!!");

    let mut a = [10, 20, 309, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index = index + 1;
    };
  
    // this approach is error prone; we could cause the program to
    // panic if the index length is incorrect. It’s also slow, because the com-
    // piler adds runtime code to perform the conditional check on every ele-
    // ment on every iteration through the loop.
    // As a more efficient alternative, you can use a for loop and execute
    // some code for each item in a collection. 

    // Arrays
    
    a = [11, 21, 310, 41, 51];

    for element in a.iter() {
        println!("print the value {}", element);
    } // no ; needed 

    println!("single int!!!!!");

    let b = 5;

    for element in 0..b { // 0..b no ()
        println!("print the value {}", element);
    } // no ; needed 

    println!("Hay caramba!!!");

    for numbero in (1..4).rev() { // excluye el numero 4, el ultimo
        println!("{}!", numbero); // es el mejor metodo por que no tienes que crear otra variable

    }

    println!("basimba!!!");


    // we’ve now increased the safety of the code and
    // eliminated the chance of bugs that might result from going beyond the
    // end of the array or not going far enough and missing some items.


    //loop {
    //    println!("again");
    //}
}
