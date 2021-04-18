use std::collections::HashMap;

fn main() {
    // let test = std::fs::read_to_string("file.txt").unwrap();
    // println!("{}", test);
    let contents = std::fs::read("file.txt").unwrap();
    let contents: Vec<u16> = contents
        .chunks(2)
        .map(|bytes| {
            let [first, second] = [bytes[0], bytes[1]];
            (first as u16) << 8 | (second as u16)
        })
        .collect();
    let contents = String::from_utf16(&contents).unwrap();
    let result = process(&contents);
    println!("{}", contents);
    let sentences = result.get("Lorem").unwrap();
    println!("------------------------------------");
    for sentence in sentences {
      println!("{}", sentence);
    }
}

fn process(contents: &str) -> HashMap<& str, Vec<& str>> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in contents.lines() {
        for sentence in line.split_terminator('.') {
            for word in sentence.trim().split_whitespace() {
                if word.chars().next().unwrap().is_uppercase() {
                    result
                        .entry(word)
                        .or_default()
                        .push(sentence);
                }
            }
        }
    }
    result
}
