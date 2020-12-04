fn walk_floors(moves: &str) -> i32 {
    let mut floor = 0;
    for m in moves.chars() {
        floor += match m {
            '(' => 1,
            ')' => -1,
            _ => {
                panic!("invalid move");
            }
        };
    }
    floor
}

fn walk_floors_until_basement(moves: &str) -> usize {
    let mut floor = 0;
    for (i, m) in moves.chars().enumerate() {
        floor += match m {
            '(' => 1,
            ')' => -1,
            _ => {
                panic!("invalid move");
            }
        };
        if floor < 0 {
            return i + 1;
        }
    }
    panic!("never got to basement");
}

fn main() {
    let contents = std::fs::read_to_string("input/1_1.txt").expect("read failed");

    dbg!(walk_floors(contents.trim()));
    dbg!(walk_floors_until_basement(contents.trim()));
}
