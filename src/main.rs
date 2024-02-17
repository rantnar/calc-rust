extern crate prettytable;
extern crate ansi_term;
use std::io::{self, Write};
mod add_invoice;
mod view_invoices;
use add_invoice::add_invoice;
use view_invoices::view_invoices;

fn main() {
    loop {
        println!("1. Add invoice");
        println!("2. Add payment");
        println!("3. View invoices");
        println!("4. Exit");
        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice, please try again");
                continue;
            }
        };

        match choice {
            1 => add_invoice(),
            2 => add_payment(),
            3 => view_invoices(),
            4 => break,
            _ => println!("Invalid choice, please try again"),
        }
    }
}

fn add_payment() {
    println!("Add payment function called");
}


