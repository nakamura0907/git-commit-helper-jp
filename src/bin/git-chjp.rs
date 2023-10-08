use clap::Parser;
use std::process;

#[derive(Parser)]
#[clap(name = "git", bin_name = "git chjp")]
#[command(version, about, long_about = None)]
struct Args {}

fn main() {
    Args::parse();

    if let Err(e) = git_commit_helper_jp::run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
