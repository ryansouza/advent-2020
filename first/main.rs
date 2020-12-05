use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries: Vec<u32> = input.lines().map(|line| line.parse::<u32>().unwrap() ).collect();

    match find_summing_entries(entries) {
        Some((x,y)) => println!("found matching entries: {} * {} = {}", x, y, x * y),
        None => println!("found no matching entries :("),
    }


    Ok(())
}

fn find_summing_entries(entries: Vec<u32>) -> Option<(u32, u32)> {
    for (idx, value) in entries.iter().enumerate() {
        let entries_to_check = &entries[idx..];

        for check in entries_to_check.iter() {
            if check + value == 2020 {
                return Some((value.clone(), check.clone()));
            }
        }
    }

    return None;
}
