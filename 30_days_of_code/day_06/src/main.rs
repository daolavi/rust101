fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<i32>().unwrap();

    for _i in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut index = 0;
        let mut string_1 = String::new();
        let mut string_2 = String::new();
        for c in input.trim().chars() {
            if index % 2 == 0 {
                string_1.push(c);
            } else {
                string_2.push(c);
            }
            index += 1;
        }
        println!("{} {}", string_1, string_2);
    }
}
