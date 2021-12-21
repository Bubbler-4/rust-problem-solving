#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<(i32,i32)> {
    input.lines().map(|s| {
        let mut w = s.split_ascii_whitespace();
        let code = match w.next().unwrap() {
            "forward" => 0,
            "down" => 1,
            _ => 2
        };
        let num: i32 = w.next().unwrap().parse().unwrap();
        (code, num)
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(i32, i32)]) -> i32 {
    let mut hor = 0;
    let mut ver = 0;
    for (code, num) in input {
        match code {
            0 => hor += num,
            1 => ver += num,
            _ => ver -= num
        };
    }
    hor * ver
}

#[aoc(day2, part2)]
pub fn part2(input: &[(i32, i32)]) -> i32 {
    let mut hor = 0;
    let mut ver = 0;
    let mut aim = 0;
    for (code, num) in input {
        match code {
            0 => { hor += num; ver += num * aim },
            1 => aim += num,
            _ => aim -= num
        };
    }
    hor * ver
}

pub const S: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&gen(S)), 150);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&gen(S)), 900);
    }
}