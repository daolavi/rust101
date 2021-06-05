fn main() {
  let mut n = String::new();
  std::io::stdin().read_line(&mut n).unwrap();
  let n = n.trim().parse::<i32>().unwrap();

  for i in 1..11 {
    println!("{} x {} = {}", n, i, n*i);
  }
}
