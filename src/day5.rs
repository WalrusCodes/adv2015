// Rules for part 1:
// * at least three vowels (aeiou only)
// * at least one letter that appears twice in a row
// * does not contain the strings ab, cd, pq, or xy

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn count_vowels(line: &str) -> usize {
    line.chars().map(|c| if is_vowel(c) { 1 } else { 0 }).sum()
}

fn contains_two_chars_twice_in_a_row(line: &str) -> bool {
    line.chars().zip(line.chars().skip(1)).any(|x| match x {
        (x1, x2) => x1 == x2,
    })
}

// Checks if given string matches our rules.
fn check_rules_part1(line: &str) -> bool {
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return false;
    }
    if count_vowels(line) < 3 {
        return false;
    }
    contains_two_chars_twice_in_a_row(line)
}

// Rules for part 2:
// * contains a pair of any two letters that appears at least twice in the string without
//   overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
// * contains at least one letter which repeats with exactly one letter between them, like xyx,
//   abcdefeghi (efe), or even aaa.

fn contains_xyx(line: &str) -> bool {
    line.chars().zip(line.chars().skip(2)).any(|x| match x {
        (x1, x2) => x1 == x2,
    })
}

fn contains_same_two_letter_pair(line: &str) -> bool {
    for i in 0..line.len() - 1 {
        for j in i + 2..line.len() - 1 {
            if line[i..=(i + 1)] == line[j..=(j + 1)] {
                return true;
            }
        }
    }
    false
}

fn check_rules_part2(line: &str) -> bool {
    contains_xyx(line) && contains_same_two_letter_pair(line)
}

fn main() {
    let contents = std::fs::read_to_string("input/5.txt").expect("read failed");
    let part1: usize = contents
        .lines()
        .map(|c| if check_rules_part1(c) { 1 } else { 0 })
        .sum();
    dbg!(part1);
    let part2: usize = contents
        .lines()
        .map(|c| if check_rules_part2(c) { 1 } else { 0 })
        .sum();
    dbg!(part2);
}
