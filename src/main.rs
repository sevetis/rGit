use clap::{Parser};
use anyhow::Result;

#[derive(Parser)]
struct Args {
    name: String
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello");
    Ok(())
}
