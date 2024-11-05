mod cli;
use structopt::StructOpt;

fn main() {
    println!("Rusty CLI");
    cli::CommandLineArgs::from_args();
}