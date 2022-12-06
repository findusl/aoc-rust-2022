use std::borrow::BorrowMut;

pub fn part_one(input: &str) -> Option<u32> {
	logic(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
	logic(input, 14)
}

fn logic(input: &str, target_len: usize) -> Option<u32> {
	let (_repeat, result) = input.chars().enumerate().fold(("".to_string(), -1), |
		(acc, result), (index, c)| {
		println!("Checking {} and {} at {}", acc, c, index);
		if result != -1 { return (acc, result) }

		let index_of = acc.find(c);
		return match index_of {
			None => {
				let mut new_sequence = acc.clone();
				new_sequence.push(c);
				if new_sequence.len() == target_len {
					(new_sequence, index as i32)
				} else {
					(new_sequence, -1)
				}
			}
			Some(index) => {
				let mut unique_part = acc[(index+1)..].to_string();
				unique_part.push(c);
				(unique_part, -1)
			}
		}
	});
	Some(result as u32 + 1)
}

fn main() {
	let input = &advent_of_code::read_file("inputs", 6);
	advent_of_code::solve!(1, part_one, input);
	advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let input = advent_of_code::read_file("examples", 6);
		assert_eq!(part_one(&input), Some(7));
	}

	#[test]
	fn test_part_two() {
		let input = advent_of_code::read_file("examples", 6);
		assert_eq!(part_two(&input), Some(19));
	}
}
