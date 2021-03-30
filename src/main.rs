use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ! indicates println! is a macro
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    //println!("The Secret Number is: {}", secret_number);
    
    loop {
    
        println!("Please input your guess.");

        // let creates a variable
        // variables are immutable by default
        // mut declares a mutable variale
        // new is an associated function, implemented on type String
        // so use ::
        let mut guess = String::new();

        // & indicates pass by reference, so read_line can change guess variable
        // references are immutable by default.  So passing &guess would not
        // change value of guess.  need to use &mut guess
        io::stdin().read_line(&mut guess)
           .expect("Failed to read line");

        // Rust allows shadowing previous guess to a new one
        // generally used to convert from one type to another
        // : inidicates annotating type
        // parse returns a Result type, so can use match to handle input
        // _ is catch-all to catch all errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
