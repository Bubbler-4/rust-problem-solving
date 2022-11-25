use itertools::Itertools;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let grid = grid_with_size(input, 50, 6);
    grid.iter().flatten().filter(|x| **x).count()
}

pub fn grid_with_size(input: &str, width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; width]; height];
    for line in input.lines() {
        let words = line.split(&[' ', 'x', '='] as &[char]).collect_vec();
        if words[0] == "rect" {
            let on_width: usize = words[1].parse().unwrap();
            let on_height: usize = words[2].parse().unwrap();
            for row in grid.iter_mut().take(on_height) {
                for cell in row.iter_mut().take(on_width) {
                    *cell = true;
                }
            }
        } else if words[1] == "row" {
            let row_id: usize = words[3].parse().unwrap();
            let offset: usize = words[5].parse().unwrap();
            grid[row_id].rotate_right(offset);
        } else {
            let col_id: usize = words[4].parse().unwrap();
            let offset: usize = words[6].parse().unwrap();
            let mut col: Vec<bool> = grid.iter().map(|r| r[col_id]).collect();
            col.rotate_right(offset);
            for (i,v) in col.iter().enumerate() {
                grid[i][col_id] = *v;
            }
        }
    }
    grid
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> String {
    let grid = grid_with_size(input, 50, 6);
    "\n".to_string() + &grid.iter().map(|r|
        r.iter().map(|c| if *c { '*' } else { ' ' }).collect::<String>()
    ).join("\n")
}

pub const SAMPLE_INPUT: &str = r"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let grid = grid_with_size(SAMPLE_INPUT, 7, 3);
        let expected = vec![
            vec![false, true, false, false, true, false, true],
            vec![true, false, true, false, false, false, false],
            vec![false, true, false, false, false, false, false]
            ];
        assert_eq!(grid, expected);
    }

    #[test]
    fn example2() {
        //assert_eq!(part2(SAMPLE_INPUT), "advent");
    }
}