use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries: Vec<i32> = input.lines().map(|line| line.parse::<i32>().unwrap() ).collect();

    // I had to switch these to passing borrowed entries
    // so that I could use it twice. I guess rust gives up ownership by default?
    match find_summing_entries(&entries) {
        Some((x,y)) => println!("found matching entries: {} * {} = {}", x, y, x * y),
        None => println!("found no matching entries :("),
    }

    match find_part2_entries(&entries) {
        Some((x,y,z)) => println!("found part 2 matching entries: {} * {} * {} = {}", x, y, z, x * y * z),
        None => println!("found no matching entries :("),
    }


    Ok(())
}

fn find_summing_entries(entries: &Vec<i32>) -> Option<(i32, i32)> {
    for value in entries.iter() {
        let needed: i32 = 2020 - value;

        if entries.contains(&needed) {
            return Some((value.clone(), needed));
        }
    }

    return None;
}



fn find_part2_entries(entries: &Vec<i32>) -> Option<(i32, i32, i32)> {
    for (idx, value1) in entries.iter().enumerate() {
        let entries_to_check = &entries[idx..];

        for value2 in entries_to_check.iter() {
            let needed: i32 = 2020 - value1 - value2;

            if entries.contains(&needed) {
                return Some((value1.clone(), value2.clone(), needed));
            }
        }
    }

    return None;
}
