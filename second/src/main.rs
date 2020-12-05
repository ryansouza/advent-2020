use std::ops::RangeInclusive;
use regex::Regex;
use lazy_static::lazy_static;
use std::convert::TryInto;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = valid_passwords_part_1(input);

    println!("Part 1 valid passwords: {}", part1);
}

pub fn valid_passwords_part_1(input: &str) -> usize {
    let entries = input.lines().map(|line| parse_line(line));
    let valid = entries.filter(|e| e.passes());

    return valid.count();
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct PasswordEntry {
    range: RangeInclusive<i32>,
    char: char,
    password: String,
}

impl PasswordEntry {
    pub fn passes(&self) -> bool {
        let matching_chars: i32 = self.password.chars().filter(|c| c == &self.char).count().try_into().unwrap();

        return self.range.contains(&matching_chars)
    }
}

lazy_static! {
    static ref PASSWORD_ENTRY_FORMAT: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

pub fn parse_line(line: &str) -> PasswordEntry {
    let captures = PASSWORD_ENTRY_FORMAT.captures(line).unwrap();

    let range: RangeInclusive<i32> = (captures[1].parse::<i32>().unwrap() ..= captures[2].parse::<i32>().unwrap());
    let char: char = captures[3].parse().unwrap();
    let password: String = captures[4].to_owned();

    return PasswordEntry{ range, char, password }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(PasswordEntry { range: (1..=3), char: 'a', password: "abcde".to_owned() }, parse_line("1-3 a: abcde"));
        assert!(PasswordEntry { range: (1..=3), char: 'a', password: "abcde".to_owned() }.passes());
        
        assert_eq!(2, valid_passwords_part_1(include_str!("../test_input.txt")));
    }
}
