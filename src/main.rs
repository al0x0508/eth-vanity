use clap::Parser;
use eth_vanity::{generator::Generator, utils::print_iterations};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short)]
    starts_with: String,
}

fn main() {
    let args = Args::parse();
    let starts_with = args.starts_with;

    println!("Starting vanity address search...");

    let generator = Generator::new(starts_with.to_string());
    std::thread::spawn({
        let total_iterations = generator.total_iterations.clone();
        move || print_iterations(total_iterations)
    });

    let start = Instant::now();
    let wallet = generator.run();

    println!(
        "\nSuccessfully found vanity address in {:?}",
        start.elapsed()
    );
    println!("Address: 0x{}", wallet.to_address_hex());
    println!("Private Key: 0x{}", wallet.key.display_secret());
}
