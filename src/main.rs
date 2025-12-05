mod day1;
mod day2;
mod day3;

pub trait DailyRiddle {
    fn name(&self) -> &str;
    fn part1(&self) -> (i128, i128) {
        return (0, 0);
    }
    fn part2(&self) -> (i128, i128) {
        return (0, 0);
    }
}

fn run_day(day: impl DailyRiddle) {
    println!("-------");
    println!("{}: ", day.name());
    let part1 = day.part1();
    println!("Part 1: Sample: {}; Riddle: {}", part1.0, part1.1);
    let part2 = day.part2();
    println!("Part 2: Sample: {}; Riddle: {}", part2.0, part2.1);
}

fn main() {
    if true //TODO: make a smarter way to select which days are executed
    {
        run_day(day1::Day1{});
    }
    if true
    {
        run_day(day2::Day2{});
    }
    if true
    {
        run_day(day3::Day3{});
    }
}
