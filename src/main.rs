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

    let regression_result = perform_regression(&features)?;
    let (_intercept, _slope, r_squared) = regression_result;

    println!(
        "Regression Coefficients: Intercept = {:.2}, Slope = {:.2}",
        regression_result.0, regression_result.1
    );

    let tree = train_decision_tree(&features, 3);
    let accuracy = evaluate_decision_tree(&tree, &features);
    println!("Decision Tree Accuracy (Predicting Attendance from Success): {:.2}", accuracy);

    println!("Regression R-squared: {:.2}", r_squared);
    Ok(())
}

