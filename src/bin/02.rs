type Input<'a> = Vec<&'a str>;

fn parse(input: &str) -> Input {
    input.lines().collect()
}

pub fn part_one(input: Input) -> Option<u32> {
    None
}

pub fn part_two(input: Input) -> Option<u32> {
    None
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 2)));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 2)));
        assert_eq!(result, None);
    }
}
