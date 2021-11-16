use md5::{Md5, Digest};

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    (0..).map(|n| {
        Md5::new().chain(input).chain(format!("{}", n)).finalize()
    }).filter(|x| &x[..3] < &[0, 0, 16]).take(8).map(|x| format!("{:x}", x[2])).collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    (0..).map(|n| {
        Md5::new().chain(input).chain(format!("{}", n)).finalize()
    }).filter(|x| &x[..3] < &[0, 0, 16])
    .scan(vec![b'_'; 8], |v, x| {
        let idx = x[2] as usize;
        let val = x[3] / 16;
        if idx < 8 && v[idx] == b'_' { v[idx] = b"0123456789abcdef"[val as usize]; }
        Some(v.clone())
    }).find(|x| x.iter().all(|&c| c != b'_')).unwrap().iter().map(|c| *c as char).collect()
}

pub const SAMPLE_INPUT: &str = r"abc";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(SAMPLE_INPUT), "18f47a30");
    }

    #[test]
    fn example2() {
        assert_eq!(part2(SAMPLE_INPUT), "05ace8e3");
    }
}