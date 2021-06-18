fn main() {
    println!("Input your integer:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let number = input.trim().parse::<i32>().unwrap();
    let reversed_number = Solution::reverse(number);
    println!("{}", reversed_number);
}

pub struct Solution {}

impl Solution {
  pub fn reverse(x: i32) -> i32 {
      let mut number = x;
      if number < 0 {
          number = -number;
      }
      
      let str = number.to_string();
      let reversed_str = str.chars().rev().collect::<String>();
      let mut result: i32;
      match reversed_str.parse::<i32>() {
        Ok(n) => result = n,
        Err(_e) => result = 0,
      };
      
      if x < 0 && result != 0 {
          result = -result;
      }
      
      result
  }
}