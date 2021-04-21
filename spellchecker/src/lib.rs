use std::usize;

pub fn spellcheck1<C: Spellchecker>(input: &str, spellchecker: C) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

pub fn spellcheck2(input: &str, spellchecker: &dyn Spellchecker) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

pub fn spellcheck3(input: &str, spellchecker: Box<dyn Spellchecker>) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

fn apply_change(string: &mut String, change: Change) {}

pub enum Change {
    Delete(std::ops::Range<usize>),
    Replace(std::ops::Range<usize>, String),
}

pub trait Spellchecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

struct NoopSpellchecker;

impl Spellchecker for NoopSpellchecker {
    #[inline(always)]
    fn check(&self, input: &str) -> Vec<Change> {
        Vec::new()
    }
}

struct AntispaceSpellchecker;

impl Spellchecker for AntispaceSpellchecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input
            .match_indices(" ")
            .map(|(index, spaces)| Change::Delete(index..index + spaces.len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let text = "Hello, is it me you're looking for?";
        let result = spellcheck1(text, NoopSpellchecker);
        assert!(result == text);
        let _ = spellcheck1(text, AntispaceSpellchecker);
        //assert!(result != text);

        let spellcheckers: Vec<&dyn Spellchecker> = vec![&NoopSpellchecker, &AntispaceSpellchecker];
        for sp in spellcheckers {
            spellcheck2(text, sp);
        }
    }
}

struct TextEditor<'a> {
  buffer: String,
  spellcheckers: Vec<&'a dyn Spellchecker>
}