use riff::riff;

fn main() {

    let lines_a = riff::lines("test_files/std_A.txt".as_ref());
    let lines_b = riff::lines("test_files/std_B.txt".as_ref());

    let delta = riff::diff(&lines_a, &lines_b);

    println!("{}", );
}
