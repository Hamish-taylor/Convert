use std::num::ParseIntError;

use clap::Parser;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    value: String,
    #[clap(short, long)]
    base: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let value = args.value;
    let base = args.base;

    let actual_base = if let Some(base) = base {
        base
    } else {
        detect_base(value.clone())
    };
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    if actual_base == 10 {
        let converted_value = from_base_10_to_base_36(value.parse::<u64>().unwrap());
        println!("{:}", converted_value);
        ctx.set_contents(converted_value).unwrap();
    } else {
        let converted_value = match from_base_36_to_base_10(value.as_str()) {
            Ok(value) => value.to_string(),
            Err(_) => "Invalid input".to_string(),
        };
        println!("{:}", converted_value);
        ctx.set_contents(converted_value.to_string()).unwrap();
    }
}

fn detect_base(value: String) -> u32 {
    match value.parse::<u64>() {
        Ok(_) => 10,
        Err(_) => 36,
    }
}

fn from_base_10_to_base_36(mut value: u64) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let mut result = Vec::new();
    let base = 36;
    let digits = "0123456789abcdefghijklmnopqrstuvwxyz";

    while value > 0 {
        let remainder = value % base;
        value /= base;
        result.push(digits.chars().nth(remainder as usize).unwrap());
    }

    result.iter().rev().collect()
}

fn from_base_36_to_base_10(value: &str) -> Result<u64, ParseIntError> {
    Ok(u64::from_str_radix(value, 36)?)
}
