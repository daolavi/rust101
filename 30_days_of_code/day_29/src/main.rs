fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<i32>().unwrap();

    for _i in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split(' ').collect();
        let n = input[0].trim().parse::<i32>().unwrap();
        let k = input[1].trim().parse::<i32>().unwrap();

        let mut max = 0;
        for a in 1..n {
            for b in a + 1..n + 1 {
                let val = a & b;
                if val > max && val < k {
                    max = val;
                }
            }
        }
        println!("{}", max);
    }
}
