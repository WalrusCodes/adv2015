use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    // Moves position based on <^>v directions.
    fn go(&mut self, c: char) {
        match c {
            '<' => {
                self.x -= 1;
            }
            '>' => {
                self.x += 1;
            }
            '^' => {
                self.y -= 1;
            }
            'v' => {
                self.y += 1;
            }
            _ => {
                panic!("invalid char: {}", c);
            }
        }
    }
}

// Calculates how many houses receive at least one present.
fn part1(input: &str) -> usize {
    let mut presents: HashSet<Pos> = HashSet::new();
    let mut pos = Pos { x: 0, y: 0 };
    for c in input.chars() {
        presents.insert(pos.clone());
        pos.go(c);
    }
    presents.insert(pos);

    presents.len()
}

// Calculates how many houses receive at least one present with 2 "cursors".
fn part2(input: &str) -> usize {
    let mut presents: HashSet<Pos> = HashSet::new();
    let mut pos = [Pos { x: 0, y: 0 }, Pos { x: 0, y: 0 }];
    for (i, c) in input.chars().enumerate() {
        presents.insert(pos[i % 2].clone());
        pos[i % 2].go(c);
    }
    presents.insert(pos[0].clone());
    presents.insert(pos[1].clone());

    presents.len()
}

fn main() {
    let contents = std::fs::read_to_string("input/3.txt")
        .expect("read failed")
        .trim()
        .to_string();

    let part1_presents = part1(&contents);
    dbg!(part1_presents);
    let part2_presents = part2(&contents);
    dbg!(part2_presents);
}
