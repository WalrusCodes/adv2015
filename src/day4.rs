use md5;

fn is_good_hash(hash: &str, zeros: usize) -> bool {
    for c in hash[0..zeros].chars() {
        if c != '0' {
            return false;
        }
    }
    true
}

fn is_good_number(key: &str, zeros: usize, num: u32) -> bool {
    let input = format!("{}{}", key, num);
    let digest = md5::compute(&input);
    is_good_hash(&format!("{:x}", digest), zeros)
}

fn find_first_good_number(key: &str, zeros: usize) -> u32 {
    let mut num = 0;
    while !is_good_number(key, zeros, num) {
        num += 1;
    }
    num
}

fn part1(key: &str) -> u32 {
    find_first_good_number(key, 5)
}

fn part2(key: &str) -> u32 {
    find_first_good_number(key, 6)
}

fn main() {
    // let key = "abcdef";
    let key = "iwrupvqb";
    dbg!(part1(key));
    dbg!(part2(key));
}
