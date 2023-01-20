#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::read_to_string;
    use crate::{common_item_in_multiple_strings, common_item_in_strings, get_points_in_line, item_to_points, string_halves, sum_points_in_lines, sum_points_in_threes, switch_ascii_cases};

    #[test]
    fn test_split_string_in_half() {
      assert_eq!(string_halves("AABB"), ("AA", "BB"));
      assert_eq!(string_halves("AABBCC"), ("AAB", "BCC"));
    }

    #[test]
    fn test_common_item_in_strings() {
        assert_eq!(common_item_in_strings(("AABB", "BCCC")), 'B');
        assert_eq!(common_item_in_strings(("abcdefg", "jklmnoc")), 'c');
        assert_eq!(common_item_in_strings(("abc", "def")), 0 as char);
    }

    #[test]
    fn test_item_to_points() {
        assert_eq!(item_to_points('A'), 1);
        assert_eq!(item_to_points('a'), 1 + 26);
        assert_eq!(item_to_points('Z'), 1 + 25);
        assert_eq!(item_to_points('z'), 1 + 26 + 25);
    }

    #[test]
    fn test_switch_ascii_cases() {
        assert_eq!(switch_ascii_cases('A'), 'a');
        assert_eq!(switch_ascii_cases('Z'), 'z');
        assert_eq!(switch_ascii_cases('a'), 'A');
        assert_eq!(switch_ascii_cases('z'), 'Z');
    }

    #[test]
    fn test_get_points_in_line() {
        assert_eq!(get_points_in_line("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22)
    }

    #[test]
    fn test_common_item_in_multiple_strings() {
        let mut data = vec! {"abcdefg", "mfjhjkjhg", "zyzyzyzyzf"};
        assert_eq!(common_item_in_multiple_strings(data[0].chars().collect::<HashSet<_>>(), data.drain(1..data.len()).collect::<Vec<_>>()), 'f')
    }

    #[test]
    fn test_sum_points_in_lines() {
        let data = read_to_string("../../day_4/test-data.txt").unwrap();
        assert_eq!(sum_points_in_lines(data.as_str()), 157)
    }

    #[test]
    fn test_sum_points_in_threes() {
        let data = read_to_string("../../day_4/test-data.txt").unwrap();
        assert_eq!(sum_points_in_threes(data.as_str()), 70)
    }
}