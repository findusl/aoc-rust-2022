use crate::RPSMoves::{PAPER, ROCK, SCISSORS};

pub fn part_one(input: &str) -> Option<u32> {
	let rounds = input.split("\n");
	let total_score = rounds.map(|round| {
		let mut chars = round.chars();
		let first_move = parse(chars.next().unwrap()).unwrap();
		let second_move = parse(chars.nth(1).unwrap()).unwrap();
		let mut sum: u32 = outcome(first_move, second_move);
		sum += move_score(second_move);
		println!("{} gives score {}", round, sum);
		sum
	}).sum::<u32>();
	Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
	let rounds = input.split("\n");
	let total_score = rounds.map(|round| {
		let mut chars = round.chars();
		let first_move = parse(chars.next().unwrap()).unwrap();
		let outcome = chars.nth(1).unwrap();
		let sum: u32 = match outcome {
			'X' /*lose*/ => match first_move {
				ROCK => 3,
				PAPER => 1,
				SCISSORS => 2
			},
			'Y' /*draw*/ => 3 + move_score(first_move),
			'Z' => 6 + match first_move {
				ROCK => 2,
				PAPER => 3,
				SCISSORS => 1,
			},
			_ => 999999999
		};
		println!("{} gives score {}", round, sum);
		sum
	}).sum::<u32>();
	Some(total_score)
}

fn parse(c: char) -> Option<RPSMoves> {
	match c {
		'A' => Some(ROCK),
		'B' => Some(PAPER),
		'C' => Some(SCISSORS),
		'X' => Some(ROCK),
		'Y' => Some(PAPER),
		'Z' => Some(SCISSORS),
		_ => None
	}
}

fn outcome(first_move: RPSMoves, second_move:RPSMoves) -> u32 {
	if first_move == second_move {
		return 3;
	}
	if (first_move == PAPER && second_move == SCISSORS) ||
		(first_move == SCISSORS && second_move == ROCK) ||
		(first_move == ROCK && second_move == PAPER)
	{
		return 6;
	}
	return 0;
}

fn move_score(played: RPSMoves) -> u32 {
	match played {
		ROCK => 1,
		PAPER => 2,
		SCISSORS => 3,
	}
}

#[derive(PartialEq, Clone, Copy)]
enum RPSMoves {
	ROCK,
	PAPER,
	SCISSORS
}

fn main() {
	let input = &advent_of_code::read_file("inputs", 2);
	advent_of_code::solve!(1, part_one, input);
	advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let input = advent_of_code::read_file("examples", 2);
		assert_eq!(part_one(&input), Some(15));
	}

	#[test]
	fn test_part_two() {
		let input = advent_of_code::read_file("examples", 2);
		assert_eq!(part_two(&input), Some(12));
	}
}
