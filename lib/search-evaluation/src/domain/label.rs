use libm::log2;
use std::collections::HashMap;

pub struct Labels {
    pub list: Vec<Label>,
}

pub struct Label {
    pub query_id: String,
    pub ideal_results: Vec<IdealResult>,
}

pub struct IdealResult {
    pub document_id: String,
    pub impairment_gain: f64,
}

impl Label {
    pub fn score(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_label_score() {
        let label = Label {
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
        relative_eq!(
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
