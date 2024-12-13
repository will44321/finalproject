#[cfg(test)]
mod test;
mod data;
mod regression;
mod decision_tree;

use data::{load_attendance_data, load_games_data, prepare_features};
use decision_tree::{train_decision_tree, evaluate_decision_tree};
use regression::perform_regression;
use std::error::Error;

