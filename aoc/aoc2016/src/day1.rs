use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    // (1, 0) = east, (0, 1) = north
    // turn left: (x, y) -> (-y, x); turn right: (x, y) -> (y, -x)
    input
        .trim()
        .split(", ")
        .scan((0, 1), |d, s| {
            *d = if &s[..1] == "L" { (-d.1, d.0) } else { (d.1, -d.0) };
            let dist: i32 = s[1..].parse().unwrap();
            Some((d.0 * dist, d.1 * dist))
        }).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[(i32, i32)]) -> i32 {
    let (x,y) = input.iter().fold((0, 0), |(a,b), (c,d)| (a+c, b+d));
    x.abs() + y.abs()
}

#[aoc(day1, part2)]
pub fn part2(input: &[(i32, i32)]) -> i32 {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut current = (0, 0);
    set.insert(current);
    for (x, y) in input {
        let dist = x.abs() + y.abs();
        let dir = (x.signum(), y.signum());
        for _ in 1..=dist {
            current.0 += dir.0;
            current.1 += dir.1;
            if set.contains(&current) {
                return current.0.abs() + current.1.abs();
            }
            set.insert(current);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator("R2, L3")), 5);
        assert_eq!(part1(&input_generator("R2, R2, R2")), 2);
        assert_eq!(part1(&input_generator("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator("R8, R4, R4, R8")), 4);
    }
}