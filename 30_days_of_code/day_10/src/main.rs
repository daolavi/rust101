fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();
    let binary_str = format!("{:b}", input);

    let mut count = 0;
    let mut max = 0;
    for c in binary_str.chars() {
        if c == '1' {
            count += 1;
        } else {
            if count > max {
                max = count;
            }
            count = 0;
        }
    }

    if count > max {
        max = count;
    }

    println!("{}", max);
}
