fn main() {

    loop {
        println!("Please select if you want to conver celcius to fahrenheit or farenheit to celcius, type cf or fc");

        let mut select_cf_fc = String::new();

        if select_cf_fc = "cf" {
            println!("Please enter how many celsius degrees you wan to convert to farenheit degrees");
            let mut c = Integrer::new();
            let f = 9/5 * c + 32;
            println!("{} CELCIUS DEGREES ARE {} FAHRENHEIT ", c, f);
            println!("Do you want to continue converting degrees in this modality? yes/no");
            let mut choose_to_quit_loser = String::new();
            if choose_to_quit_loser = "yes" {
                println!("Go hug your mother");
                break
            }
            else {
            }
        }

        else if select_cf_fc = "fc" {
            println!("Please enter how many farenheit degrees you want to convert to celsius degrees");
            let mut f = Integrer::new();
            let c = 5/9 * f - 32;
            println!("{} FAHRENHEIT DEGREES ARE {} CELCIUS", f, c);
        }
        else {
            println!("No valid output");
        }

    }
}
