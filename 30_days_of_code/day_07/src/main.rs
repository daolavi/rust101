fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let _n = input.trim().parse::<i32>();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let array : Vec<&str> = input.trim().split(' ').collect();
    let mut reversed_array = array.clone();
    reversed_array.reverse();

    for element in reversed_array.iter() {
      print!("{} ", element);
    }
}
