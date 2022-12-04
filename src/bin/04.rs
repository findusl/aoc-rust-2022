pub fn part_one(input: &str) -> Option<u32> {
	let rucksacks = input.lines();
	let amount = rucksacks.filter(|rucksack| {
		let (first_start, first_end, second_start, second_end) = parse_pair(rucksack);
		let is_included = (first_start <= second_start && first_end >= second_end) ||
			(first_start >= second_start && first_end <= second_end);
		is_included
	}).count();
	Some(amount as u32)
}

fn parse_pair(pair: &str) -> (u32, u32, u32, u32) {
	// TODO would love to be able to destructure the split directly. is there something?
	let mut pair_iter = pair.split(',');
	let (first_start, first_end) = parse_range(pair_iter.next().unwrap());
	let (second_start, second_end) = parse_range(pair_iter.next().unwrap());
	// TODO is there a better way to combine these?
	(first_start, first_end, second_start, second_end)
}

fn parse_range(range: &str) -> (u32, u32) {
	let mut split = range.split('-');
	(split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
	let rucksacks = input.lines();
	let amount = rucksacks.filter(|rucksack| {
		let (first_start, first_end, second_start, second_end) = parse_pair(rucksack);
		let has_overlap = (first_start <= second_start && first_end >= second_start) ||
			(first_start >= second_start && first_start <= second_end);
		has_overlap
	}).count();
	Some(amount as u32)
}

fn main() {
	let input = &advent_of_code::read_file("inputs", 4);
	advent_of_code::solve!(1, part_one, input);
	advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let input = advent_of_code::read_file("examples", 4);
		assert_eq!(part_one(&input), Some(2));
	}

	#[test]
	fn test_part_two() {
		let input = advent_of_code::read_file("examples", 4);
		assert_eq!(part_two(&input), Some(4));
	}
}
