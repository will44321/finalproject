#[cfg(test)]
mod tests {
    use crate::data::{Attendance, Game, prepare_features};
    use crate::regression::perform_regression;
    use crate::decision_tree::{train_decision_tree, evaluate_decision_tree};

    #[test]
    fn test_prepare_features() {
        let attendance_data = vec![
            Attendance {
                team: String::from("Boston University"),
                team_name: String::from("Terriers"),
                year: 2020,
                week: 1,
                weekly_attendance: Some(70000.0),
            },
            Attendance {
                team: String::from("Northeastern"),
                team_name: String::from("Huskies"),
                year: 2020,
                week: 1,
                weekly_attendance: Some(65000.0),
            },
        ];

        let games_data = vec![
            Game {
                year: 2020,
                week: 1,
                home_team: String::from("Terriers"),
                away_team: String::from("Huskies"),
                winner: String::from("Terriers"),
            },
            Game {
                year: 2020,
                week: 1,
                home_team: String::from("Eagles"),
                away_team: String::from("Beavers"),
                winner: String::from("Beavers"),
            },
        ];

        let features = prepare_features(&attendance_data, &games_data);
        assert_eq!(features.len(), 2);
        assert_eq!(features[0], (70000.0, 1.0));
        assert_eq!(features[1], (65000.0, 0.0));
    }

    #[test]
    fn test_perform_regression() {
        let features = vec![
            (70000.0, 1.0),
            (65000.0, 0.0),
            (80000.0, 1.0),
            (75000.0, 1.0),
        ];

        let result = perform_regression(&features).unwrap();
        assert!(result.0 > 0.0);
        assert!(result.1 > 0.0);
        assert!(result.2 >= 0.0);
    }

    #[test]
    fn test_decision_tree() {
        let features = vec![
            (70000.0, 1.0),
            (65000.0, 0.0),
            (80000.0, 1.0),
            (75000.0, 1.0),
        ];

        let tree = train_decision_tree(&features, 3);
        let accuracy = evaluate_decision_tree(&tree, &features);

        assert!(accuracy >= 0.0 && accuracy <= 1.0); // Accuracy is a valid percentage
    }
}
