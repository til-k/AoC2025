
fn solve1(input: &str) -> i64 {
    let mut solution: i64 = 0;
    for line in input.lines() {
        let jolts = line.chars().map(|c| {c.to_digit(10).unwrap_or(0)}).collect::<Box<[u32]>>(); //TODO: fn should probably always return result so that we dont hide a possible error
        let jolts_without_last = &jolts[..jolts.len()-1];
        if let Some(highest_val) = jolts_without_last.iter().max() {
            if let Some(highest_index) = jolts.iter().position(|v| return v == highest_val) {//this does a second pass which is wasteful
                let jolts_after_highest = &jolts[highest_index+1..];
                if let Some(second_highest_val) = jolts_after_highest.iter().max() {
                    solution += (highest_val * 10 + second_highest_val) as i64;
                }
            }
        }
    }
    return solution as i64;
}

fn solve2(input: &str) -> i128 {
    let mut solution: i128 = 0;
    for line in input.lines() {
        let jolts = line.chars().map(|c| {c.to_digit(10).unwrap_or(0)}).collect::<Box<[u32]>>();
        let mut min_index = 0;
        for batt_offset in (0..12).rev() {
            let jolts_truncated = &jolts[min_index..jolts.len()-batt_offset];
            if let Some(highest_val) = jolts_truncated.iter().max() {
                if let Some(highest_index) = jolts_truncated.iter().position(|v| return v == highest_val) {
                    min_index += highest_index+1;
                    solution += (10_u64.pow(batt_offset as u32) * (*highest_val) as u64) as i128;
                }
            }
        }
    }
    return solution;
}

pub fn part1() -> (i64, i64) {
    return (solve1(include_str!("sample.txt")), solve1(include_str!("riddle.txt")));
}

pub fn part2() -> (i128, i128) {
    return (solve2(include_str!("sample.txt")), solve2(include_str!("riddle.txt")));
}