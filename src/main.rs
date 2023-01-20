use std::fs;
use fs::read_to_string;
use std::collections::HashSet;

mod test;

fn string_halves(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

fn common_item_in_strings(strings: (&str, &str)) -> char {
    let search_set = strings.0.chars().collect::<HashSet<_>>();
    for character in strings.1.chars() {
        if search_set.contains(&character) {
            return character;
        }
    }
    return 0 as char;
}

fn common_item_in_multiple_strings(lookup: HashSet<char>, mut strings: Vec<&str>) -> char {
    if lookup.len() == 0 {
        return 0 as char;
    }

    if strings.len() == 0 {
        return **lookup.iter().collect::<Vec<_>>().first().unwrap();
    }

    let mut a: HashSet<char> = HashSet::new();

    for character in strings.first().unwrap().chars() {
        if lookup.contains(&character) {
            a.insert(character);
        }
    }

    return common_item_in_multiple_strings(a, strings.drain(1..strings.len()).collect::<Vec<_>>());
}

fn item_to_points(item: char) -> usize {
    if item.is_ascii_lowercase() {
        return item as usize - 64 - 6;
    }
    item as usize - 64
}

fn switch_ascii_cases(item: char) -> char {
    if item.is_ascii_lowercase() {
        return item.to_ascii_uppercase()
    }
    item.to_ascii_lowercase()
}

fn split_into_sets_of_threes(lines: Vec<&str>) -> Vec<Vec<&str>> {
    lines.chunks(3).map(|x| x.to_vec()).collect::<Vec<_>>()
}

fn get_points_in_line(line: &str) -> usize {
    item_to_points(switch_ascii_cases(common_item_in_strings(string_halves(line))))
}

fn sum_points_in_lines(lines: &str) -> usize {
    lines.split_ascii_whitespace().map(get_points_in_line).sum()
}

fn get_points_in_threes(mut lines: Vec<&str>) -> usize {
    item_to_points(switch_ascii_cases(common_item_in_multiple_strings(lines[0].chars().collect::<HashSet<_>>(), lines.drain(1..lines.len()).collect::<Vec<_>>())))
}

fn sum_points_in_threes(lines: &str) -> usize {
    split_into_sets_of_threes(lines.split_ascii_whitespace().collect::<Vec<&str>>()).iter().map(|chunk| get_points_in_threes(chunk.clone())).sum()
}

fn main() {
    let data = read_to_string("../../day_4/final-data.txt").unwrap();
    println!("PART 1: {}", sum_points_in_lines(data.as_str()));
    println!("PART 2: {}", sum_points_in_threes(data.as_str()));
}
