use std::error::Error;
use csv::Reader;
use statrs::statistics::Statistics; // Import Statistics trait
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("Fraud Detection Dataset.csv")?;

    let mut transaction_amounts = Vec::new();
    let mut fraud_counts = [0, 0]; // Fraudulent (1) vs. Non-Fraudulent (0)
    let mut transaction_types = HashMap::new();
    let mut missing_values = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        
        // Transaction Amount
        if let Some(val) = record.get(2) {
            if let Ok(num) = val.parse::<f64>() {
                transaction_amounts.push(num);
            } else {
                *missing_values.entry("Transaction_Amount").or_insert(0) += 1;
            }
        }

        // Fraudulent Column
        if let Some(val) = record.get(11) {
            if let Ok(num) = val.parse::<i32>() {
                fraud_counts[num as usize] += 1;
            }
        }

        // Transaction Type Frequency
        if let Some(t_type) = record.get(3) {
            *transaction_types.entry(t_type.to_string()).or_insert(0) += 1;
        }
    }

    if transaction_amounts.is_empty() {
        println!("No valid transaction data found!");
        return Ok(());
    }

    // Use `Statistics` trait methods
    let mean = transaction_amounts.mean();
    let median = transaction_amounts.median();
    let mean = transaction_amounts.mean();
    let median = transaction_amounts.median();
    let std_dev = transaction_amounts.std_dev();

    println!("== Basic Statistics ==");
    println!("Transaction Amount: Mean = {:.2}, Median = {:.2}, Std Dev = {:.2}",
        mean.unwrap_or(0.0), median.unwrap_or(0.0), std_dev.unwrap_or(0.0));

    println!("\n== Fraudulent Transactions ==");
    println!("Non-Fraudulent: {}", fraud_counts[0]);
    println!("Fraudulent: {}", fraud_counts[1]);

    println!("\n== Transaction Type Frequency ==");
    for (t_type, count) in transaction_types.iter() {
        println!("{}: {}", t_type, count);
    }

    println!("\n== Missing Values ==");
    for (col, count) in missing_values.iter() {
        println!("{}: {}", col, count);
    }

    Ok(())
}