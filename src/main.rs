use clap::Parser;
use eth_vanity::{cli::Cli, generator::Generator, utils::print_iterations};

fn main() {
    let args = Cli::parse();
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
