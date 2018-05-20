#[macro_use]
extern crate structopt;

use std::collections::HashSet;
use std::string::String;
use std::io::stdin;
use std::io::BufRead;
use std::time::{Instant, Duration};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "uniqcache")]
struct Args {
    /// Time between cache clears, in seconds
    #[structopt(short = "t", long = "clear-time", default_value = "1")]
    clear_time: u64
}

fn main() {
    let args = Args::from_args();

    let mut cache = HashSet::<String>::new();
    let mut now = Instant::now();
    let stdin = stdin();

    for line in stdin.lock().lines().map(|s| s.unwrap_or("".into())) {
        if !cache.contains(&line.clone()) {
            cache.insert(line.clone());
            println!("{}", line);
        }

        if now.elapsed() > Duration::from_secs(args.clear_time) {
            cache.clear();
            now = Instant::now();
        }
    }
}
