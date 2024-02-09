use chrono::{NaiveDate, Duration};
use rand::Rng;
use std::fs::File;
use std::io::{Write, BufWriter};

fn generate(rng: &mut rand::rngs::ThreadRng, num_rows: u32, start_date: NaiveDate, end_date: NaiveDate, mut csv_file: BufWriter<File>) {
    writeln!(csv_file, "Transaction ID,Product ID,Quantity,Unit Price,Total Price,Date").expect("Could not write header to the file.");

    for i in 0..num_rows {
        let transaction_id = i+1;
        let product_id = rng.gen_range(1..=10_000);
        let quantity = rng.gen_range(1..=100);
        let unit_price: f64 = rng.gen::<f64>() * rng.gen_range(100..=1000) as f64;
        let unit_price = unit_price.round() as u32;
        let total_price = quantity as u32 * unit_price;
        let days_in_range = (end_date - start_date).num_days();
        let random_day_gap = rng.gen_range(0..days_in_range);
        let random_date = start_date + Duration::days(random_day_gap);
        writeln!(csv_file, "{},{},{},{},{},{}", transaction_id, product_id, quantity, unit_price, total_price, random_date).expect("Cannot write line to the file.");
    }

    csv_file.flush().unwrap();
}

fn main() {
    let start = std::time::Instant::now();
    let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).expect("Could not create start_date!");
    let end_date = NaiveDate::from_ymd_opt(2024, 2, 9).expect("Could not create end_date!");
    let mut rng = rand::thread_rng();
    let file = File::create("mock_data.csv").expect("Could not create the CSV file.");
    let csv_file = BufWriter::new(file);
    generate(&mut rng, 10_00_00_000, start_date, end_date, csv_file);
    println!("Time taken: {:?}", start.elapsed());
}