pub fn part_one(input: &str) -> Option<&str> {

	None
}

pub fn part_two(input: &str) -> Option<u32> {
	None
}

fn process(input: &str) -> Vec<Vec<char>> {
	let mut parts = input.split("\n\n");
	let initial_tower = parse_tower(parts.next().unwrap());
	None.unwrap()
}

fn parse_tower(tower_input: &str) -> Vec<Vec<char>> {
	let lines: Vec<&str> = tower_input.lines().collect();
	let length = lines.last().unwrap().len()/4 + 1;
	None.unwrap()
}

fn main() {
	let input = &advent_of_code::read_file("inputs", 5);
	advent_of_code::solve!(1, part_one, input);
	advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let input = advent_of_code::read_file("examples", 5);
		assert_eq!(part_one(&input), Some("CMZ"));
	}

	#[test]
	fn test_part_two() {
		let input = advent_of_code::read_file("examples", 5);
		assert_eq!(part_two(&input), None);
	}
}
