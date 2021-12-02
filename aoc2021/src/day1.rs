#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .flat_map(|x| x.parse())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> usize {
    input.windows(2).filter(|&x| x[0] < x[1]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> usize {
    input.windows(4).filter(|&x| x[0] < x[3]).count()
}

pub const S: &str = "199
200
208
210
200
207
240
269
260
263";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&gen(S)), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&gen(S)), 5);
    }
}