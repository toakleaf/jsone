extern crate globwalk;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// blob[s] of json files
    #[arg()]
    arg_blobs: Vec<String>,

    /// blob[s] of json files
    #[arg(short, long)]
    blobs: Vec<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        "./",
        &[&args.blobs[..], &args.arg_blobs[..]].concat(),
    )
    .build()
    .into_iter();
    for entries in walker {
        for entry in entries {
            match entry {
                Ok(path) => println!("{:?}", path.file_name()),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
