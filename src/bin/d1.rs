fn main() {
    let input = include_str!("../../inputs/day1.1").lines();
    let total: u32 = input.map(parse_line_digit).sum();
    // println!("{total}");
    assert_eq!(54927, total); // valid

    let input = include_str!("../../inputs/day1.2").lines();
    let total: u32 = input.map(parse_line_elven_digit).sum();
    assert_eq!(54581, total); // valid
    // println!("{total}");
}

fn parse_line_digit(line: &str) -> u32 {
    let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
    format!("{first}{last}").parse::<u32>().unwrap()
}

fn parse_line_elven_digit(line: &str) -> u32 {
    let first = find_digit_ltr(line);
    let last = find_digit_rtl(line);
    // println!("first={first}, last={last}");
    let merged = format!("{first}{last}");
    // println!("{line:50} {merged}");
    merged.parse::<u32>().unwrap()
}

fn find_digit_ltr(line: &str) -> &str {
    let len = line.len();
    let mut chars = line.chars();
    let mut index = 0;

    loop {
        match chars
            .next()
            .expect("we should have found a first digit before EOL")
        {
            '0'..='9' => {
                return &line[index..index + 1];
            }
            'o' if len - index >= 3 && &line[index..index + 3] == "one" => return "1",
            't' if len - index >= 3 && &line[index..index + 3] == "two" => return "2",
            't' if len - index >= 5 && &line[index..index + 5] == "three" => return "3",
            'f' if len - index >= 4 && &line[index..index + 4] == "four" => return "4",
            'f' if len - index >= 4 && &line[index..index + 4] == "five" => return "5",
            's' if len - index >= 3 && &line[index..index + 3] == "six" => return "6",
            's' if len - index >= 5 && &line[index..index + 5] == "seven" => return "7",
            'e' if len - index >= 5 && &line[index..index + 5] == "eight" => return "8",
            'n' if len - index >= 4 && &line[index..index + 4] == "nine" => return "9",
            c => {
                index += c.len_utf8();
            }
        }
    }
}

fn find_digit_rtl(line: &str) -> &str {
    let len = line.len();
    let mut chars = line.chars().rev(); // note the 'rev()'
    let mut index = len;

    loop {
        match chars
            .next()
            .expect("we should have found a first digit before EOL")
        {
            '0'..='9' => {
                return &line[index - 1..index];
            }
            'e' if index.checked_sub(3).is_some() && &line[index - 3..index] == "one" => return "1",
            'o' if index.checked_sub(3).is_some() && &line[index - 3..index] == "two" => return "2",
            'e' if index.checked_sub(5).is_some() && &line[index - 5..index] == "three" => return "3",
            'r' if index.checked_sub(4).is_some() && &line[index - 4..index] == "four" => return "4",
            'e' if index.checked_sub(4).is_some() && &line[index - 4..index] == "five" => return "5",
            'x' if index.checked_sub(3).is_some() && &line[index - 3..index] == "six" => return "6",
            'n' if index.checked_sub(5).is_some() && &line[index - 5..index] == "seven" => return "7",
            't' if index.checked_sub(5).is_some() && &line[index - 5..index] == "eight" => return "8",
            'e' if index.checked_sub(4).is_some() && &line[index - 4..index] == "nine" => return "9",
            c => {
                index -= c.len_utf8();
            }
        }
    }
}
