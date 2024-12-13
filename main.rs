#[cfg(test)]
mod test;
mod data;
mod regression;
mod decision_tree;

use data::{load_attendance_data, load_games_data, prepare_features};
use decision_tree::{train_decision_tree, evaluate_decision_tree};
use regression::perform_regression;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let attendance_data = load_attendance_data("attendance.csv")?;
    let games_data = load_games_data("games.csv")?;

    let features = prepare_features(&attendance_data, &games_data);
    if features.is_empty() {
        println!("No valid data found for analysis.");
        return Ok(());
    }
    Ok(())
}