fn solve1(input: &str) -> i64 {
    let mut password: i32 = 0;
    let mut position: u8 = 50;
    for l in input.lines() {
        let (direction, amount) = l.split_at(1);
        let turns = (amount.parse::<u16>().unwrap_or_else(|_| {println!("Amount not valid"); return 0;}) % 100) as u8;
        match direction.chars().next() {
            Some(c) => match c {
                'R' => {
                    if position + turns <= 99 {position += turns}
                    else {position = position + turns - 100}
                },
                'L' => {
                    if position >= turns {position -= turns}
                    else {position = 100 - (turns - position)}
                },
                _ => ()
            },
            None => ()
        }
        if position == 0 {password += 1};
    }
    return password as i64;

}

fn solve2(input: &str) -> i64 {
    let mut password: i32 = 0;
    let mut position: u8 = 50;
    for l in input.lines() {
        let (direction, amount) = l.split_at(1);
        let actual_turns = amount.parse::<u32>().unwrap_or_else(|_| {println!("Amount not valid"); return 0;});
        password += (actual_turns / 100) as i32;
        let turns = (actual_turns % 100) as u8;
        match direction.chars().next() {
            Some(c) => match c {
                'R' => {
                    if position + turns <= 99 {position += turns}
                    else {position = position + turns - 100; password += 1}
                },
                'L' => {
                    if position >= turns {position -= turns; if position == 0 {password += 1}}
                    else {if position != 0 {password += 1}; position = 100 - (turns - position)}
                },
                _ => ()
            },
            None => ()
        }
    }
    return password as i64;

}

pub fn part1() -> (i64, i64) {
    return (solve1(include_str!("sample.txt")), solve1(include_str!("riddle.txt")));
}

pub fn part2() -> (i64, i64) {
    return (solve2(include_str!("sample.txt")), solve2(include_str!("riddle.txt")));
}