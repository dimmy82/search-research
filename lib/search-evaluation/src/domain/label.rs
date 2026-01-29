use libm::log2;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Labels {
    pub list: Vec<Label>,
}

#[derive(Debug, PartialEq)]
pub struct Label {
    pub query_id: String,
    pub ideal_results: Vec<IdealResult>,
}

#[derive(Debug, PartialEq)]
pub struct IdealResult {
    pub document_id: String,
    pub impairment_gain: f64,
}

impl Label {
    pub fn score(&mut self) -> f64 {
        self.sort_ideal_results();
        let mut score = 0.0;
        for (index, ir) in self.ideal_results.iter().enumerate() {
            score += log2(index as f64 + 2.0) * ir.impairment_gain;
        }
        score
    }

    fn sort_ideal_results(&mut self) {
        self.ideal_results.sort_by(|left, right| {
            right
                .impairment_gain
                .partial_cmp(&left.impairment_gain)
                .unwrap_or(Ordering::Equal)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_label_score() {
        let mut label = Label {
            query_id: "query1".to_string(),
            ideal_results: vec![
                IdealResult {
                    document_id: "doc_1".to_string(),
                    impairment_gain: 0.5,
                },
                IdealResult {
                    document_id: "doc_2".to_string(),
                    impairment_gain: 0.2,
                },
                IdealResult {
                    document_id: "doc_3".to_string(),
                    impairment_gain: 1.0,
                },
                IdealResult {
                    document_id: "doc_4".to_string(),
                    impairment_gain: 0.2,
                },
                IdealResult {
                    document_id: "doc_5".to_string(),
                    impairment_gain: 0.5,
                },
                IdealResult {
                    document_id: "doc_6".to_string(),
                    impairment_gain: 1.0,
                },
            ],
        };
        assert_relative_eq!(
            label.score(),
            log2(2.0) * 1.0
                + log2(3.0) * 1.0
                + log2(4.0) * 0.5
                + log2(5.0) * 0.5
                + log2(6.0) * 0.2
                + log2(7.0) * 0.2
        );
    }
}
