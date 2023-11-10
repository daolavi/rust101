fn main() {
    let lines = read_lines(String::from("./tests.txt"));
    let price: f64 = lines[0].parse().unwrap();
    let tip: f64 = lines[1].parse().unwrap();
    let tax: f64 = lines[2].parse().unwrap();
    let total = (price + price * tip / 100.0 + price * tax / 100.0).round();
    println!("{total}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: String) -> Vec<String> {
    let contents =
        std::fs::read_to_string(filename).expect("Should have been able to read the file");

    let array = contents.lines().map(|s| s.to_string()).collect();
    array
}
