use std::error::Error;

pub fn perform_regression(features: &[(f64, f64)]) -> Result<(f64, f64, f64), Box<dyn Error>> {
    if features.is_empty() {
        println!("No data points available for regression.");
        return Ok((0.0, 0.0, 0.0));
    }

    let (x, y): (Vec<f64>, Vec<f64>) = features.iter().map(|(attendance, wins)| (*wins, *attendance)).unzip();

    let n = x.len();
    let x_mean = x.iter().sum::<f64>() / n as f64;
    let y_mean = y.iter().sum::<f64>() / n as f64;

    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (&x_i, &y_i) in x.iter().zip(y.iter()) {
        numerator += (x_i - x_mean) * (y_i - y_mean);
        denominator += (x_i - x_mean).powi(2);
    }

    if denominator.abs() < f64::EPSILON {
        println!("Data variance is too low for meaningful regression.");
        return Ok((0.0, 0.0, 0.0));
    }

    let slope = numerator / denominator;
    let intercept = y_mean - slope * x_mean;

    let mut ss_total = 0.0;
    let mut ss_residual = 0.0;
    for (&x_i, &y_i) in x.iter().zip(y.iter()) {
        let y_pred = intercept + slope * x_i;
        ss_total += (y_i - y_mean).powi(2);
        ss_residual += (y_i - y_pred).powi(2);
    }

    if ss_total.abs() < f64::EPSILON {
        println!("No variability in the dependent variable (attendance).");
        return Ok((intercept, slope, 0.0));
    }

    let r_squared = 1.0 - (ss_residual / ss_total);

    Ok((intercept, slope, r_squared))
}

