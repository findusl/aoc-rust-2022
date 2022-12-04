pub fn part_one(input: &str) -> Option<u32> {
	let rucksacks = input.lines();
	let sum_priorities = rucksacks.map(|rucksack| duplicate_item_value(rucksack)).sum::<u32>();
	return Some(sum_priorities)
}

fn duplicate_item_value(rucksack: &str) -> u32 {
	let first_compartment = &rucksack[..rucksack.len()/2];
	let second_compartment = &rucksack[rucksack.len()/2..];
	let items_first_compartment: [u32; 53] = item_array(first_compartment);
	let duplicate = second_compartment.chars()
		.find(|&item| items_first_compartment[item_score(item) as usize] != 0);
	return item_score(duplicate.unwrap())
}

fn item_array(rucksack: &str) -> [u32; 53] {
	let mut rucksack_items: [u32; 53] = [0; 53];
	rucksack.chars().for_each(|item| {
		rucksack_items[item_score(item) as usize] += 1;
	});
	return rucksack_items
}

fn item_score(item: char) -> u32 {
	let ascii = (item as u32);
	return if ascii >= 97 {
		ascii - 96
	} else {
		ascii - 38
	}
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut rucksacks = input.lines().peekable();
	let mut sum: u32 = 0;
	while rucksacks.peek().is_some() {
		let first_items = item_array(rucksacks.next().unwrap());
		let second_items = item_array(rucksacks.next().unwrap());
		let shared_item_char = rucksacks.next().unwrap().chars()
			.find(|&item| {
				let position = item_score(item) as usize;
				first_items[position] != 0 && second_items[position] != 0
			});
		sum += item_score(shared_item_char.unwrap())
	}
	Some(sum)
}

fn main() {
	let input = &advent_of_code::read_file("inputs", 3);
	advent_of_code::solve!(1, part_one, input);
	advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let input = advent_of_code::read_file("examples", 3);
		assert_eq!(part_one(&input), Some(157));
	}

	#[test]
	fn test_part_two() {
		let input = advent_of_code::read_file("examples", 3);
		assert_eq!(part_two(&input), Some(70));
	}
}
