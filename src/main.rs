pub mod day1;
pub mod day2;

fn main() {
    {
        println!("-------");
        println!("Day 1: ");
        let part1 = day1::part1();
        println!("Part 1: Sample: {}; Riddle: {}", part1.0, part1.1);
        let part2 = day1::part2();
        println!("Part 2: Sample: {}; Riddle: {}", part2.0, part2.1);
    }
    {
        println!("-------");
        println!("Day 2: ");
        let part1 = day2::part1();
        println!("Part 1: Sample: {}; Riddle: {}", part1.0, part1.1);
        let part2 = day2::part2();
        println!("Part 2: Sample: {}; Riddle: {}", part2.0, part2.1);
    }
}
