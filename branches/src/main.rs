fn main() {
    let number = 39;

    if number < 1 {
    println!("condition was true");
    } 

    else {
    println!("condition was false");
    }

    if number % 1 == 0 {

    println!("number is divisible by 1");
    } 
    
    if number % 2 == 0 {
    println!("number is divisible by 2");
    } 
    
    if number % 3 == 0 {
    println!("number is divisible by 3");
    } 

    if number % 4 == 0 {
        println!("number is divisible by 4");
        } 

    if number % 5 == 0 {
        println!("number is divisible by 5");
        } 

    if number % 6 == 0 {
        println!("number is divisible by 6");
        } 

    if number % 7 == 0 {
        println!("number is divisible by 7");
        } 
    
    if number % 8 == 0 {
        println!("number is divisible by 8");
        } 

    if number % 9 == 0 {
        println!("number is divisible by 9");
        } 
            
    else {
    println!("number is not divisible by 9") ;
    }

    // Using too many else if expressions can clutter your code, so if you
    // have more than one, you might want to refactor your code.

    //When this program executes, it checks each if expression in turn and
    //executes the first body for which the condition holds true. Note that
    //even though 6 is divisible by 2, we don’t see the output number is
    //divisible by 2, nor do we see the number is not divisible by 4,
    //3, or 2 text from the else block. The reason is that Rust will only
    //execute the block for the first true condition, and once it finds one, it
    //won’t even check the rest.

    let condition = true ;
    let numero = if condition { // es como si dijera si la condicion verdadera = 5
        5
    } else {
        6
    };

    println!("The number is {}", numero);

    // The number variable will be bound to a value based on the outcome of the if expression.
    // Remember that blocks of code evaluate to the last expression in them,
    // and numbers by themselves are also expressions. In this case, the value
    // of the whole if expression depends on which block of code executes.

    // both of the expresions have to evaluate to an integer, variables must have a single type
    // the compiler would be more complex and would make fewer guarantees
    // about the code if it had to keep track of multiple hypothetical types
    // for any variable.

}

