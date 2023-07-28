use std::fs::read_to_string;

fn main() {
    println!("Calculating the score for the Rock, Paper, Scissors tournament based on the strategy guide!");

    calculate_tournament_score();
}

fn calculate_tournament_score() {
    let mut total_score: u16 = 0;

    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "A X" {
            total_score += 3;
        } else if line == "A Y" {
            total_score += 4;
        } else if line == "A Z" {
            total_score += 8;
        } else if line == "B X" {
            total_score += 1;
        } else if line == "B Y" {
            total_score += 5;
        } else if line == "B Z" {
            total_score += 9;
        } else if line == "C X" {
            total_score += 2;
        } else if line == "C Y" {
            total_score += 6;
        } else if line == "C Z" {
            total_score += 7;
        }
    }

    println!("Total Score: {total_score}");
}