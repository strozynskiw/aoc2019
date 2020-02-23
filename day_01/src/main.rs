use std::fs;

fn part_1(input: &str) -> i32 {
    input.lines().map(|l| l.parse::<i32>().unwrap()).fold(0, |acc, fuel| acc + (fuel/3)-2)
}

fn compute_required_fuel(mass: i32) -> i32 {
    let fuel = (mass/3)-2;
    if fuel < 0 {
        return 0
    }
    else {
        return fuel + compute_required_fuel(fuel);
    }
}

fn part_2(input: &str) -> i32 {
    input.lines()
        .map(|l| l.parse::<i32>().unwrap())
        .fold(0, |acc, mass| acc + compute_required_fuel(mass))
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    print!("Result1: {}\n", part_1(&content));
    print!("Result2: {}\n", part_2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1("12"));
        assert_eq!(2, part_1("14"));
        assert_eq!(654, part_1("1969"));
        assert_eq!(33583, part_1("100756"));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(2, part_2("12"));
        assert_eq!(966, part_2("1969"));
        assert_eq!(50346, part_2("100756"));
    }
}
