use riff::diff;
use std::env;
use riff::diff::DiffRep;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please supply at least two file names");
        return;
    }

    let file_a = &args[1];
    let file_b = &args[2];

    let lines_a = diff::lines(file_a.as_ref());
    let lines_b = diff::lines(file_b.as_ref());

    println!("{}", diff::diff(&lines_a,&lines_b).to_diff());
}
