use clap::Parser;
use globset::{Glob, GlobBuilder};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let builder = GlobBuilder::new(&args.name);

    let set = builder.build().expect("ok");

    println!("{:?}", set.compile_matcher());

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
