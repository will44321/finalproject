use csv::Reader;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Attendance {
    #[allow(dead_code)]
    pub team: String,
    pub team_name: String,
    pub year: u16,
    pub week: u8,
    #[serde(deserialize_with = "deserialize_attendance")]
    pub weekly_attendance: Option<f64>,
}

fn deserialize_attendance<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s.as_deref() {
        Some("NA") | None => Ok(None),
        Some(value) => value.parse::<f64>().map(Some).map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub year: u16,
    pub week: u8,
    pub winner: String,
    #[allow(dead_code)]
    pub home_team: String,
    #[allow(dead_code)]
    pub away_team: String,
}

pub fn load_attendance_data(path: &str) -> Result<Vec<Attendance>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(record) => data.push(record),
            Err(err) => {
                println!("Failed to parse attendance record: {}", err);
            }
        }
    }
    Ok(data)
}

pub fn load_games_data(path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut games = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(record) => games.push(record),
            Err(e) => {
                println!("Failed to parse game record: {}", e);
            }
        }
    }
    Ok(games)
}

pub fn prepare_features(
    attendance_data: &[Attendance],
    games_data: &[Game],
) -> Vec<(f64, f64)> {
    let mut features = Vec::new();
    for attendance in attendance_data {
        if let Some(att) = attendance.weekly_attendance {
            let wins = games_data
                .iter()
                .filter(|game| {
                    game.winner == attendance.team_name
                        && game.year == attendance.year
                        && game.week == attendance.week
                })
                .count() as f64;
            features.push((att, wins));
	}
    }
    features
}
