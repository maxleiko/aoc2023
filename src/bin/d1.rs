fn main() {
    let input = include_str!("../../inputs/day1").lines();
    let total: u32 = input.map(parse_line).sum();
    // println!("{total}");
    assert_eq!(54927, total); // valid
}

fn parse_line(line: &str) -> u32 {
    let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
    format!("{first}{last}").parse::<u32>().unwrap()
}
