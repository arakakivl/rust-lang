// I/O library and rand lib.
use std::{cmp::Ordering, io::{self, Write}};
use rand::Rng;

fn main() {
    // Generating a random number with the "rand" library.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Shh... secret_number is {secret_number}.");

    println!("Guess the number!");
    
    // Looping until the right result is provided
    

    // Looping until the right result is provided.
    loop {
         // Getting guess and flushing stdout in order to print without a new line.
        print!("Tell me your guess: ");
        io::stdout().flush().expect("Failed to flush.");

        /* 
            Creating a variable to store user input. By default, variables are immutable; its value won't change.
            Adding the "mut" keyword makes the variable a mutable one. Then, assigning a new "String" instance to the "guess" mutable variable.
            "String" is a type provided by the standard library that is a growable, UTF-8 encoded bit of text.
            The "new" function call is associated w/ the "String" type, and it creates a new and empty string.
        */

        let mut guess = String::new();

        /*
            Calling the "stdin" function from the "io" module for handling user input. This function returns
            an instance of "Stdin" type that represents a handle to the standard input for your terminal.
            
            After that, the "read_line" function is being called to get input from the user w/ the recently created 
            "guess" variable for appending the input into it.
            
            The "&" means a reference, and a reference allows different parts of the code to access the variable without
            copying it into memory multiple times. Because references are immutable by default, the "mut" keyword is used again.

            The "read_line" function returns a "Result" type value, that is an "Enumeration" or "enum". The "Ok" and "Err" are variants of 
            the "Result" enum, which "Ok" contains the returned data and "Err" contains information about the error.

            The "expect" method is a method binded to the "Result" type.
        */

        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        println!("You guessed: {guess}");

        // Parsing the "guess" variable to a "u32" type.
        // Using the same variable by shadowing it (thus not needing to create another one).
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // Comparing the "guess" variable w/ the "secret_number" variable.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => { 
                println!("Too big!");
                break;   
            },
            Ordering::Equal => println!("You win!")
        }
    }
}