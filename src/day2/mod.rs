fn is_valid_product_code1(val: i64) -> bool {
    let as_str = val.to_string();
    for i in 1..((as_str.len()/2)+1) {
        let (first, second) = as_str.split_at(i);
        if second.eq(first) { return false; }
    }
    return true;
}

fn solve1(input: &str) -> i64 {
    let mut solution: i64 = 0;
    let input_no_nl = input.replace("\n", "").replace("\r", "");
    for id_range in input_no_nl.split(",") {
        let (range_start, range_end) = id_range.split_at(id_range.find("-").unwrap_or(0));
        let range_end_without_dash = &range_end[1..];
        if let Ok(start) = range_start.parse::<i64>() && let Ok(end) = range_end_without_dash.parse::<i64>(){
            for i in start..end+1 {
                if !is_valid_product_code1(i) {solution+=i as i64;}
            }
        }
    }
    return solution as i64;
}

fn is_valid_product_code2(val: i64) -> bool {
    let as_str = val.to_string();
    for i in 1..((as_str.len()/2)+1) {
        let (first, second) = as_str.split_at(i);
        let seq_len = first.len();
        if second.len() >= seq_len && second.len() % seq_len == 0 { //only need to check cases where len of rest of string is fully dividable
            let mut sequence_only_repeats = true;
            for j in 0..(second.len() / seq_len) {
                if !second[seq_len*j..seq_len*(j+1)].eq(first) {sequence_only_repeats = false;}
            }
            if sequence_only_repeats {return false;}
        }
    }
    return true;
}

fn solve2(input: &str) -> i64 {
    let mut solution: i64 = 0;
    let input_no_nl = input.replace("\n", "").replace("\r", "");
    for id_range in input_no_nl.split(",") {
        let (range_start, range_end) = id_range.split_at(id_range.find("-").unwrap_or(0));
        let range_end_without_dash = &range_end[1..];
        if let Ok(start) = range_start.parse::<i64>() && let Ok(end) = range_end_without_dash.parse::<i64>(){
            for i in start..end+1 {
                if !is_valid_product_code2(i) {solution+=i as i64}
            }
        }
    }
    return solution as i64;
}

pub fn part1() -> (i64, i64) {
    return (solve1(include_str!("sample.txt")), solve1(include_str!("riddle.txt")));
}

pub fn part2() -> (i64, i64) {
    return (solve2(include_str!("sample.txt")), solve2(include_str!("riddle.txt")));
}