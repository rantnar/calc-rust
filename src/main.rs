extern crate ansi_term;
use std::io::{self, Write};
mod add_invoice;
mod view_invoices;
mod add_payment;
use add_invoice::add_invoice;
use view_invoices::view_invoices;
use add_payment::handle_add_payment;

fn main() -> io::Result<()>{
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
            2 => handle_add_payment()?,
            3 => view_invoices(),
            4 => return Ok(()),
            _ => println!("Invalid choice, please try again"),
        }
}

}


