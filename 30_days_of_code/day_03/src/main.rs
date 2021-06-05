fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    if n % 2 == 1 || (n % 2 == 0 && n > 6 && n <= 20) {
        println!("Weird");
    } else if n % 2 == 0 && ((n >= 2 && n <= 5) || n > 20) {
        println!("Not Weird");
    }
}
