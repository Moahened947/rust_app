use std::io;
use chrono::{Utc, Datelike};

fn main() {
    loop {
        println!("Enter your birth year (type 'close' to exit):");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim leading and trailing whitespaces
        let trimmed_input = input.trim();

        // Check if the user wants to exit
        if trimmed_input.eq_ignore_ascii_case("close") {
            println!("Exiting the program. Goodbye!");
            break;
        }

        // Parse input to an integer
        let birth_year: i32 = match trimmed_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid year.");
                continue;
            }
        };

        // Get the current year
        let current_year = Utc::now().year();

        // Calculate age
        let age = current_year - birth_year;

        // Display the result
        println!("Your age is: {}", age);
    }
}
