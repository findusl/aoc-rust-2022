pub fn part_one(input: &str) -> Option<u32> {
	println!("Input ist {}", input);
	let max_calories = input.split("\n\n")
		.map(|person| {
			person.split("\n").map(|food| food.parse::<u32>().unwrap()).sum()
		}).max();
	max_calories
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut all_values: Vec<u32> = input.split("\n\n")
		.map(|person| {
			person.split("\n").map(|food| food.parse::<u32>().unwrap()).sum()
		}).collect();
	all_values.sort();
	all_values.reverse();
	let top_3_calories = all_values.iter().take(3).sum::<u32>();
	Some(top_3_calories)
}

fn main() {
	let input = &advent_of_code::read_file("inputs", 1);
	advent_of_code::solve!(1, part_one, input);
	advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let input = advent_of_code::read_file("examples", 1);
		assert_eq!(part_one(&input), Some(24000));
	}

	#[test]
	fn test_part_two() {
		let input = advent_of_code::read_file("examples", 1);
		assert_eq!(part_two(&input), Some(45000));
	}
}
