use libm::log2;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Labels {
    pub list: Vec<Label>,
}

#[derive(Debug, PartialEq)]
pub struct Label {
    query_id: String,
    ideal_results: Vec<IdealResult>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdealResult {
    pub document_id: String,
    pub impairment_gain: f64,
}

impl Label {
    pub fn new(query_id: String, ideal_results: Vec<IdealResult>) -> Self {
        let mut ideal_results_cloned = ideal_results.clone();
        ideal_results_cloned.sort_by(|left, right| {
            right
                .impairment_gain
                .partial_cmp(&left.impairment_gain)
                .unwrap_or(Ordering::Equal)
        });
        Self {
            query_id,
            ideal_results: ideal_results_cloned,
        }
    }

    pub fn score(&self) -> f64 {
        let mut score = 0.0;
        for (index, ir) in self.ideal_results.iter().enumerate() {
            println!("score: {}", score);
            score += 1.0 / log2(index as f64 + 2.0) * ir.impairment_gain;
            println!(
                "add: {} * {}",
                1.0 / log2(index as f64 + 2.0),
                ir.impairment_gain
            )
        }
        println!("score: {}", score);
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_new() {
        let actual = Label::new(
            "query1".to_string(),
            vec![
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
        );
        assert_eq!(
            actual,
            Label {
                query_id: "query1".to_string(),
                ideal_results: vec![
                    IdealResult {
                        document_id: "doc_3".to_string(),
                        impairment_gain: 1.0,
                    },
                    IdealResult {
                        document_id: "doc_6".to_string(),
                        impairment_gain: 1.0,
                    },
                    IdealResult {
                        document_id: "doc_1".to_string(),
                        impairment_gain: 0.5,
                    },
                    IdealResult {
                        document_id: "doc_5".to_string(),
                        impairment_gain: 0.5,
                    },
                    IdealResult {
                        document_id: "doc_2".to_string(),
                        impairment_gain: 0.2,
                    },
                    IdealResult {
                        document_id: "doc_4".to_string(),
                        impairment_gain: 0.2,
                    },
                ]
            }
        )
    }

    #[test]
    fn test_label_score() {
        let label = Label {
            query_id: "query1".to_string(),
            ideal_results: vec![
                IdealResult {
                    document_id: "doc_3".to_string(),
                    impairment_gain: 1.0,
                },
                IdealResult {
                    document_id: "doc_6".to_string(),
                    impairment_gain: 1.0,
                },
                IdealResult {
                    document_id: "doc_1".to_string(),
                    impairment_gain: 0.5,
                },
                IdealResult {
                    document_id: "doc_5".to_string(),
                    impairment_gain: 0.5,
                },
                IdealResult {
                    document_id: "doc_2".to_string(),
                    impairment_gain: 0.2,
                },
                IdealResult {
                    document_id: "doc_4".to_string(),
                    impairment_gain: 0.2,
                },
            ],
        };
        assert_relative_eq!(
            label.score(),
            1.0 / log2(2.0) * 1.0
                + 1.0 / log2(3.0) * 1.0
                + 1.0 / log2(4.0) * 0.5
                + 1.0 / log2(5.0) * 0.5
                + 1.0 / log2(6.0) * 0.2
                + 1.0 / log2(7.0) * 0.2
        );
    }
}
