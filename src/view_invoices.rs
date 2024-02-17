use serde_json::Value;
use std::fs;
use std::io::{self, BufRead};

pub fn view_invoices() {
    let mut invoices = vec![];
    let invoice_files = fs::read_dir("invoices/singles").unwrap();
    for file in invoice_files {
        let file = file.unwrap().path();
        if file.is_file() {
            let invoice: Value = serde_json::from_reader(fs::File::open(file).unwrap()).unwrap();
            invoices.push(invoice);
        }
    }

    let mut filter = String::new();
    let mut page = 0;
    loop {
        println!("Enter invoice ID to filter, > to go to next page, < to go to previous page:");
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        if input.trim() == ">" {
            page += 1;
        } else if input.trim() == "<" {
            if page > 0 {
                page -= 1;
            }
        } else {
            filter = input.trim().to_string();
            page = 0;
        }

        let filtered_invoices: Vec<&Value> = invoices.iter().filter(|invoice| {
            invoice["invoice_id"].as_str().unwrap().contains(&filter)
        }).collect();
        let paged_invoices: Vec<&Value> = filtered_invoices.chunks(10).nth(page).unwrap_or(&[]).to_vec();
        for invoice in &paged_invoices {
            println!("{}", invoice);
        }
    }
}

