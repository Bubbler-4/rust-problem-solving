#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| line
            .split_ascii_whitespace()
            .flat_map(|w| w.parse()).collect()
        ).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<i32>]) -> usize {
    input.iter().filter(|&x| x[0] + x[1] + x[2] > x[0].max(x[1]).max(x[2]) * 2).count()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<i32>]) -> usize {
    input.chunks_exact(3).map(|chunk|
        (0..3).filter(|&i| chunk[0][i] + chunk[1][i] + chunk[2][i] > chunk[0][i].max(chunk[1][i]).max(chunk[2][i]) * 2).count()
    ).sum()
}

pub const SAMPLE_INPUT: &str = r"1 1 1
1 1 3
1 3 1
3 1 1
2 3 2
4 3 2";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT)), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(SAMPLE_INPUT)), 4);
    }
}