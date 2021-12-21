use std::collections::HashSet;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    input.lines().filter(|line| {
        let mut found_outside = false;
        let mut found_inside = false;
        for (i, v) in line.split(&['[', ']'] as &[char]).enumerate() {
            let bytes = v.as_bytes();
            if bytes.windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1]) {
                if i % 2 == 0 { found_outside = true; }
                else { found_inside = true; }
            }
        }
        found_outside && !found_inside
    }).count()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|line| {
        let mut found_outside = HashSet::<(u8,u8)>::new();
        let mut found_inside = HashSet::<(u8,u8)>::new();
        for (i, v) in line.split(&['[', ']'] as &[char]).enumerate() {
            let bytes = v.as_bytes();
            for w in bytes.windows(3) {
                if w[0] == w[2] && w[0] != w[1] {
                    if i % 2 == 0 { found_outside.insert((w[0], w[1])); }
                    else { found_inside.insert((w[1], w[0])); }
                }
            }
        }
        !found_outside.is_disjoint(&found_inside)
    }).count()
}

pub const SAMPLE_INPUT: &str = r"abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";

pub const SAMPLE_INPUT2: &str = r"aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(SAMPLE_INPUT), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(SAMPLE_INPUT2), 3);
    }
}