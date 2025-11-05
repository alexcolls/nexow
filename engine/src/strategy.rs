use crate::types::*;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;

pub trait Strategy: Send + Sync {
    fn train(&mut self, bars: &[Bar]) -> anyhow::Result<()>;
    fn decide(&self, last: &Bar) -> bool; // true = long, false = flat
}

pub struct RfStrategy {
    pub trees: usize,
    pub max_depth: usize,
    model: Option<RandomForestClassifier<f64, usize>>,
}

impl RfStrategy {
    pub fn new(trees: usize, max_depth: usize) -> Self {
        Self {
            trees,
            max_depth,
            model: None,
        }
    }
}

impl Strategy for RfStrategy {
    fn train(&mut self, bars: &[Bar]) -> anyhow::Result<()> {
        if bars.len() < 100 {
            return Ok(());
        }

        // Features: [return, volatility proxy, momentum]
        let mut x = Vec::with_capacity(bars.len().saturating_sub(2) * 3);
        let mut y = Vec::with_capacity(bars.len().saturating_sub(2));

        for w in bars.windows(3) {
            let r1 = (w[1].close / w[0].close) - 1.0;
            let r2 = (w[2].close / w[1].close) - 1.0;
            let vol = (w[2].high - w[2].low) / w[2].close;
            x.extend_from_slice(&[r1, vol, w[2].close - w[1].close]);
            y.push(if r2 > 0.0 { 1 } else { 0 });
        }

        let m = DenseMatrix::from_2d_array(&vec![x.chunks(3).map(|c| c.to_vec()).collect::<Vec<_>>()]);
        let model = RandomForestClassifier::fit(&m, &y, Default::default())?;
        self.model = Some(model);
        Ok(())
    }

    fn decide(&self, last: &Bar) -> bool {
        // Fallback heuristic if model not trained
        if self.model.is_none() {
            return last.close > last.open;
        }

        // Simple single-bar inference using last bar features
        let feat = vec![vec![
            0.0,
            (last.high - last.low) / last.close,
            last.close - last.open,
        ]];
        let m = DenseMatrix::from_2d_array(&feat);
        let pred = self.model.as_ref().unwrap().predict(&m).unwrap_or(vec![0]);
        pred[0] == 1
    }
}
