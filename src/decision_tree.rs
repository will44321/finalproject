#[derive(Clone, Debug)]
pub struct DecisionNode {
    pub feature: usize,
    pub threshold: f64,
    pub left: Option<Box<DecisionNode>>,
    pub right: Option<Box<DecisionNode>>,
    pub value: Option<f64>,
}

impl DecisionNode {
    pub fn predict(&self, sample: &[f64]) -> f64 {
        if let Some(value) = self.value {
            return value;
        }

        if sample[self.feature] < self.threshold {
            self.left.as_ref().unwrap().predict(sample)
        } else {
            self.right.as_ref().unwrap().predict(sample)
        }
    }
}

pub fn train_decision_tree(data: &[(f64, f64)], depth: usize) -> DecisionNode {
    if depth == 0 || data.len() <= 1 {
        let avg_value = data.iter().map(|&(_, y)| y).sum::<f64>() / data.len() as f64;
        return DecisionNode {
            feature: 0,
            threshold: 0.0,
            left: None,
            right: None,
            value: Some(avg_value),
        };
    }

    let feature = 0;
    let threshold = data.iter().map(|&(x, _)| x).sum::<f64>() / data.len() as f64;
    let (left, right): (Vec<_>, Vec<_>) = data.iter().partition(|&&(x, _)| x < threshold);

    let left_node = train_decision_tree(&left, depth - 1);
    let right_node = train_decision_tree(&right, depth - 1);

    DecisionNode {
        feature,
        threshold,
        left: Some(Box::new(left_node)),
        right: Some(Box::new(right_node)),
        value: None,
    }
}

pub fn evaluate_decision_tree(tree: &DecisionNode, features: &[(f64, f64)]) -> f64 {
    let predictions: Vec<f64> = features.iter().map(|(x, _)| tree.predict(&[*x])).collect();

    let mse: f64 = predictions
        .iter()
        .zip(features.iter())
        .map(|(pred, (_, actual))| (pred - actual).powi(2))
        .sum::<f64>()
        / features.len() as f64;
    let rmse = mse.sqrt();
    1.0 - rmse
}
