use std::io::{self, Write};
use serde_json::json;
use std::fs::OpenOptions;
use std::fs;
use std::path::Path;

pub fn add_invoice() {
    let mut invoice_id = String::new();
    loop {
        invoice_id.clear();
        print!("Please enter the invoice ID: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut invoice_id).unwrap();
        invoice_id = invoice_id.trim().to_string();
        if invoice_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '/') {
            break;
        } else {
            println!("Invalid invoice ID, please try again");
        }
    }

    let mut invoice_currency = String::new();
    loop {
        invoice_currency.clear();
        print!("Please enter the invoice currency (USD, GBP, EUR, PLN): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut invoice_currency).unwrap();
        invoice_currency = invoice_currency.trim().to_uppercase();
        if ["USD", "GBP", "EUR", "PLN"].contains(&invoice_currency.as_str()) {
            break;
        } else {
            println!("Invalid currency, please try again");
        }
    }

    let mut date_issued = String::new();
    loop {
        date_issued.clear();
        print!("Please enter the date issued (YYYY-MM-DD): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut date_issued).unwrap();
        date_issued = date_issued.trim().to_string();
        if date_issued.chars().filter(|c| *c == '-').count() == 2 && date_issued.len() == 10 {
            break;
        } else {
            println!("Invalid date, please try again");
        }
    }

    let mut invoice_amount = String::new();
    loop {
        invoice_amount.clear();
        print!("Please enter the invoice amount: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut invoice_amount).unwrap();
        match invoice_amount.trim().parse::<f32>() {
            Ok(_) => break,
            Err(_) => println!("Invalid amount, please try again"),
        }
    }

    println!("Invoice added: ID = {}, Currency = {}, Date Issued = {}, Amount = {}", invoice_id, invoice_currency, date_issued, invoice_amount);
    let invoice = json!({
        "invoice_id": invoice_id,
        "invoice_currency": invoice_currency,
        "date_issued": date_issued,
        "invoice_amount": invoice_amount.trim().parse::<f32>().unwrap(),
        "invoice_paid": false,
        "payments": []
    });
    fs::create_dir_all("invoices/singles").unwrap();
    let file_path = format!("invoices/singles/{}.json", invoice_id);
if Path::new(&file_path).exists() {
    eprintln!("Invoice with ID {} already exists.", invoice_id);
    return;
}
let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(file_path)
    .unwrap();

    if let Err(e) = writeln!(file, "{}", invoice) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
