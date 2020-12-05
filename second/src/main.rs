use regex::Regex;
use lazy_static::lazy_static;
use std::convert::TryInto;
use std::ops::BitXor;


fn main() {
    let input = include_str!("../input.txt");
    let part1 = valid_passwords_part_1(input);
    let part2 = valid_passwords_part_2(input);

    println!("Part 1 valid passwords: {}", part1);
    println!("Part 2 valid passwords: {}", part2);
}

pub fn valid_passwords_part_1(input: &str) -> usize {
    let entries = input.lines().map(|line| parse_line(line));
    let valid = entries.filter(|e| e.passes_part1());

    return valid.count();
}

pub fn valid_passwords_part_2(input: &str) -> usize {
    let entries = input.lines().map(|line| parse_line(line));
    let valid = entries.filter(|e| e.passes_part2());

    return valid.count();
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct PasswordEntry {
    num1: i32,
    num2: i32,
    char: char,
    password: String,
}

impl PasswordEntry {
    pub fn passes_part1(&self) -> bool {
        let matching_chars: i32 = self.password.chars().filter(|c| c == &self.char).count().try_into().unwrap();

        return matching_chars >= self.num1 && matching_chars <= self.num2
    }

    pub fn passes_part2(&self) -> bool {
        let char1 = self.password.chars().nth((self.num1 - 1) as usize).unwrap() == self.char;
        let char2 = self.password.chars().nth((self.num2 - 1 ) as usize).unwrap() == self.char;

        return char1 ^ char2
    }
}

lazy_static! {
    static ref PASSWORD_ENTRY_FORMAT: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

pub fn parse_line(line: &str) -> PasswordEntry {
    let captures = PASSWORD_ENTRY_FORMAT.captures(line).unwrap();

    let num1 = captures[1].parse::<i32>().unwrap();
    let num2 = captures[2].parse::<i32>().unwrap();
    let char: char = captures[3].parse().unwrap();
    let password: String = captures[4].to_owned();

    return PasswordEntry{ num1, num2, char, password }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(PasswordEntry { num1: 1, num2: 3, char: 'a', password: "abcde".to_owned() }, parse_line("1-3 a: abcde"));
        assert!(PasswordEntry { num1: 1, num2: 3, char: 'a', password: "abcde".to_owned() }.passes_part1());

        assert_eq!(2, valid_passwords_part_1(include_str!("../test_input.txt")));
        assert_eq!(524, valid_passwords_part_1(include_str!("../input.txt")));

        assert!(  PasswordEntry { num1: 1, num2: 3, char: 'a', password: "abcde".to_owned() }.passes_part2());
        assert!(! PasswordEntry { num1: 1, num2: 3, char: 'b', password: "cdefg".to_owned() }.passes_part2());

        assert_eq!(485, valid_passwords_part_2(include_str!("../input.txt")));
    }
}
