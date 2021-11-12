#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<(i32, i32)>> {
    // (1, 0) = right, (0, 1) = down
    input
        .trim()
        .split('\n')
        .map(|line| line
            .chars()
            .map(|c| match c {
                'L' => (-1, 0),
                'R' => (1, 0),
                'U' => (0, -1),
                'D' => (0, 1),
                _ => unreachable!()
            }).collect()
        ).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<(i32, i32)>]) -> String {
    input.iter().scan((1, 1), |cur, v| {
        for (x, y) in v {
            cur.0 += x;
            cur.1 += y;
            if cur.0 < 0 { cur.0 = 0; }
            if cur.0 > 2 { cur.0 = 2; }
            if cur.1 < 0 { cur.1 = 0; }
            if cur.1 > 2 { cur.1 = 2; }
        }
        Some(format!("{}", cur.0 + cur.1 * 3 + 1))
    }).collect()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<(i32, i32)>]) -> String {
    input.iter().scan((0, 2), |cur, v| {
        for (x, y) in v {
            let next = (cur.0 + x, cur.1 + y);
            if (next.0 - 2).abs() + (next.1 - 2).abs() <= 2 { *cur = next; }
        }
        let idx = match cur.1 {
            0 => 1,
            1 => 1 + cur.0,
            2 => 5 + cur.0,
            3 => 9 + cur.0,
            4 => 13,
            _ => unreachable!()
        };
        Some(format!("{:X}", idx))
    }).collect()
}

pub const SAMPLE_INPUT: &str = r"ULL
RRDDD
LURDL
UUUUD";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT)), "1985".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(SAMPLE_INPUT)), "5DB3".to_string());
    }
}