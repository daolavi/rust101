fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    let value = factorial(n);
    println!("{}", value);
}

fn factorial(n : i32) -> i32 {
  if n <= 1 {
    return 1;
  }

  n * factorial(n - 1)
}
