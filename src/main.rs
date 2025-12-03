pub mod day1;
pub mod day2;
pub mod day3;

fn main() {
    if false //TODO: make a smarter way to select which days are executed
    {
        println!("-------");
        println!("Day 1: ");
        let part1 = day1::part1();
        println!("Part 1: Sample: {}; Riddle: {}", part1.0, part1.1);
        let part2 = day1::part2();
        println!("Part 2: Sample: {}; Riddle: {}", part2.0, part2.1);
    }
    if false
    {
        println!("-------");
        println!("Day 2: ");
        let part1 = day2::part1();
        println!("Part 1: Sample: {}; Riddle: {}", part1.0, part1.1);
        let part2 = day2::part2();
        println!("Part 2: Sample: {}; Riddle: {}", part2.0, part2.1);
    }
    if true
    {
        println!("-------");
        println!("Day 3: ");
        let part1 = day3::part1();
        println!("Part 1: Sample: {}; Riddle: {}", part1.0, part1.1);
        let part2 = day3::part2();
        println!("Part 2: Sample: {}; Riddle: {}", part2.0, part2.1);
    }
}
