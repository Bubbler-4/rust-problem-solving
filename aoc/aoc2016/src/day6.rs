#[aoc(day6, part1)]
pub fn part1(input: &str) -> String {
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let rowlen = lines[0].len();
    (0..rowlen).map(|i| {
        let mut freqs = vec![0; 26];
        for row in &lines {
            freqs[row[i] as usize - 97] += 1;
        }
        let max = freqs.iter().max().unwrap();
        let idx = freqs.iter().position(|x| x == max).unwrap();
        (idx + 97) as u8 as char
    }).collect()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> String {
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let rowlen = lines[0].len();
    (0..rowlen).map(|i| {
        let mut freqs = vec![0; 26];
        for row in &lines {
            freqs[row[i] as usize - 97] += 1;
        }
        let min = freqs.iter().filter(|x| **x != 0).min().unwrap();
        let idx = freqs.iter().position(|x| x == min).unwrap();
        (idx + 97) as u8 as char
    }).collect()
}

pub const SAMPLE_INPUT: &str = r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(SAMPLE_INPUT), "easter");
    }

    #[test]
    fn example2() {
        assert_eq!(part2(SAMPLE_INPUT), "advent");
    }
}