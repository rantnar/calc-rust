use std::fs;
use std::io::{self, Write};
use serde_json::{Value, Map};
use chrono::NaiveDate;

pub fn handle_add_payment() -> io::Result<()> {
    print!("Enter invoice id: ");
    io::stdout().flush().unwrap();
    let mut invoice_id = String::new();
    io::stdin().read_line(&mut invoice_id).unwrap();
    match add_payment(&invoice_id.trim()) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error adding payment: {}", e);
            Err(e)
        },
    }
}

pub fn add_payment(invoice_id: &str) -> io::Result<()> {
    let file_path = format!("invoices/singles/{}.json", invoice_id);
    let file_content = fs::read_to_string(&file_path)?;
    let mut invoice: Value = serde_json::from_str(&file_content)?;

    if invoice["invoice_paid"].as_bool().unwrap_or(true) {
        println!("Invoice is already paid.");
        return Ok(());
    }

    let mut payment_amount: f64;
    let mut date_payment: NaiveDate;
    loop {
        print!("Enter payment amount: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        payment_amount = input.trim().parse().unwrap();

        print!("Enter payment date (YYYY-MM-DD): ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        date_payment = NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d").unwrap();

        if date_payment < NaiveDate::parse_from_str(invoice["date_issued"].as_str().unwrap(), "%Y-%m-%d").unwrap() {
            println!("Payment date is earlier than the invoice issued date. Please try again.");
            continue;
        }

        let total_payments: f64 = invoice["payments"].as_array().unwrap().iter().map(|p| p["payment_amount"].as_f64().unwrap()).sum();
        if payment_amount + total_payments > invoice["invoice_amount"].as_f64().unwrap() {
            println!("Payment amount exceeds the invoice amount. Please try again.");
            continue;
        }

        break;
    }

    let mut payments = invoice["payments"].as_array().unwrap().clone();
    let mut payment = Map::new();
    payment.insert("payment_amount".to_string(), Value::from(payment_amount));
    payment.insert("date_payment".to_string(), Value::from(date_payment.format("%Y-%m-%d").to_string()));
    payments.push(Value::Object(payment));
    invoice["payments"] = Value::Array(payments);

    let total_payments: f64 = invoice["payments"].as_array().unwrap().iter().map(|p| p["payment_amount"].as_f64().unwrap()).sum();
    if total_payments == invoice["invoice_amount"].as_f64().unwrap() {
        invoice["invoice_paid"] = Value::from(true);
    }

    fs::write(file_path, serde_json::to_string_pretty(&invoice)?)?;
    Ok(())
}