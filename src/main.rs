use riff::diff;
use riff::io;

fn main() {

    let lines_a = io::lines("test_files/std_A.txt".as_ref());
    let lines_b = io::lines("test_files/std_B.txt".as_ref());

    let delta = diff::diff(&lines_a, &lines_b);

    println!("{:?}", delta);
}
