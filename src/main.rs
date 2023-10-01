use clap::Parser;
use eth_vanity::{generator::Generator, utils::print_iterations};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short)]
    starts_with: String,

    /// Number of addresses to generate
    #[arg(short, default_value = "1")]
    count: u64,
}

fn main() {
    let args = Args::parse();
    let starts_with = args.starts_with;
    let count = args.count;

    println!("Starting vanity address search...");

    let generator = Generator::new(starts_with.to_string(), count);
    std::thread::spawn({
        let total_iterations = generator.total_iterations.clone();
        move || print_iterations(total_iterations)
    });

    generator.run();
}
