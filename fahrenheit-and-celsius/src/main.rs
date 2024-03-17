use std::io;

fn main() {
    loop {
        println!("Please select if you want to convert Celsius to Fahrenheit or Fahrenheit to Celsius, type cf or fc");

        let mut select_cf_fc = String::new();
        io::stdin().read_line(&mut select_cf_fc).expect("Failed to read line");

        if select_cf_fc.trim() == "cf" {
            println!("Please enter how many Celsius degrees you want to convert to Fahrenheit degrees");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("Failed to read line");
            let c: f32 = c.trim().parse().expect("Please enter a valid number");

            let f = (9.0/5.0) * c + 32.0;
            println!("{} CELSIUS DEGREES ARE {} FAHRENHEIT ", c, f);
        } else if select_cf_fc.trim() == "fc" {
            println!("Please enter how many Fahrenheit degrees you want to convert to Celsius degrees");
            let mut f = String::new();
            io::stdin().read_line(&mut f).expect("Failed to read line");
            let f: f32 = f.trim().parse().expect("Please enter a valid number");

            let c = (5.0/9.0) * (f - 32.0);
            println!("{} FAHRENHEIT DEGREES ARE {} CELSIUS", f, c);
        } else {
            println!("No valid output");
        }

        println!("Do you want to continue converting degrees in this modality? yes/no");
        let mut choose_to_quit = String::new();
        io::stdin().read_line(&mut choose_to_quit).expect("Failed to read line");
        if choose_to_quit.trim() != "yes" {
            println!("Go hug your mother");
            break;
        }
    }
}

// Tried to read user input into select_cf_fc, c, and f, but not actually reading any input. 
// use std::io::stdin().read_line(&mut select_cf_fc) to read user input.

// Comparing the select_cf_fc variable, but it's empty because haven't 
// read anything into it.

// Attempted to parse integers from strings using i32::new(), 
// which is incorrect. 
// parse() method to convert strings to integers.