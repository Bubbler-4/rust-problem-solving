use itertools::Itertools;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Vec<String>, u32, String)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut words: Vec<_> = line.split(&['-', '[', ']'][..]).map(|s| s.to_string()).collect();
            words.pop();
            let key = words.pop().unwrap();
            let val = words.pop().unwrap().parse().unwrap();
            (words, val, key)
        }).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Vec<String>, u32, String)]) -> u32 {
    input.iter().filter(|&(x, _, z)| {
        let mut cnts: Vec<i32> = vec![0; 26];
        for s in x {
            for c in s.chars() {
                cnts[c as u8 as usize - 97] += 1;
            }
        }
        let mut idxs: Vec<u8> = (0..26).collect();
        idxs.sort_by_key(|&i| -cnts[i as usize]);
        idxs[..5].iter().map(|i| (i + 97) as char).collect::<String>() == *z
    }).map(|(_, y, _)| y).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Vec<String>, u32, String)]) -> String {
    // List of all decrypted rooms plus sector ID, joined by newlines
    input.iter().filter(|&(x, _, z)| {
        let mut cnts: Vec<i32> = vec![0; 26];
        for s in x {
            for c in s.chars() {
                cnts[c as u8 as usize - 97] += 1;
            }
        }
        let mut idxs: Vec<u8> = (0..26).collect();
        idxs.sort_by_key(|&i| -cnts[i as usize]);
        idxs[..5].iter().map(|i| (i + 97) as char).collect::<String>() == *z
    }).flat_map(|(x, y, _)| {
        let decrypted: String = x.iter().map(
            |s| s.chars().map(|c| ((c as u32 - 97 + y) % 26 + 97) as u8 as char).collect::<String>()
        ).join(" ");
        if decrypted.contains("north") { Some(format!("{} {}", decrypted, y)) } else { None }
    }).join("\n")
}

pub fn part2_mini(input: &[(Vec<String>, u32, String)]) -> String {
    // List of all decrypted rooms plus sector ID, joined by newlines
    input.iter().filter(|&(x, _, z)| {
        let mut cnts: Vec<i32> = vec![0; 26];
        for s in x {
            for c in s.chars() {
                cnts[c as u8 as usize - 97] += 1;
            }
        }
        let mut idxs: Vec<u8> = (0..26).collect();
        idxs.sort_by_key(|&i| -cnts[i as usize]);
        idxs[..5].iter().map(|i| (i + 97) as char).collect::<String>() == *z
    }).flat_map(|(x, y, _)| {
        let decrypted: String = x.iter().map(
            |s| s.chars().map(|c| ((c as u32 - 97 + y) % 26 + 97) as u8 as char).collect::<String>()
        ).join(" ");
        if true { Some(format!("{} {}", decrypted, y)) } else { None }
    }).join("\n")
}

pub const SAMPLE_INPUT: &str = r"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

pub const SAMPLE_INPUT2: &str = r"qzmt-zixmtkozy-ivhz-343[zimth]";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT)), 1514);
    }

    #[test]
    fn example2() {
        assert_eq!(part2_mini(&input_generator(SAMPLE_INPUT2)), "very encrypted name 343");
    }
}