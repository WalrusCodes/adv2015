#[derive(Debug)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    // Parses "29x13x26" into a Present.
    fn parse(s: &str) -> Option<Self> {
        let mut split = s.trim().split('x').map(|n| n.parse::<u32>());
        let l = split.next()?.ok()?;
        let w = split.next()?.ok()?;
        let h = split.next()?.ok()?;
        Some(Present { l, w, h })
    }

    fn calculate_wrapping_paper(self: &Self) -> u32 {
        let sides = vec![self.l * self.w, self.w * self.h, self.h * self.l];
        let smallest = sides.iter().min().unwrap();

        sides.iter().fold(0, |sz, side| sz + 2 * side) + smallest
    }

    fn calculate_ribbon(self: &Self) -> u32 {
        let sides = vec![self.l + self.w, self.w + self.h, self.h + self.l];
        let smallest = sides.iter().min().unwrap();
        smallest * 2 + self.l * self.w * self.h
    }
}

fn main() {
    let contents = std::fs::read_to_string("input/2.txt").expect("read failed");
    let presents = contents
        .lines()
        .map(|line| Present::parse(line).unwrap())
        .collect::<Vec<Present>>();
    let total_paper = presents
        .iter()
        .fold(0, |sum, present| sum + present.calculate_wrapping_paper());
    dbg!(total_paper);
    let total_ribbon = presents
        .iter()
        .fold(0, |sum, present| sum + present.calculate_ribbon());
    dbg!(total_ribbon);
}
