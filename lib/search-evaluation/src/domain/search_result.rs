use crate::domain::label::Labels;
use libm::log2;

#[derive(Debug, PartialEq)]
pub struct SearchResults {
    pub list: Vec<SearchResult>,
}

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub query_id: String,
    pub document_ids: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct EvaluationResult {
    pub i_dcg: f64,
    pub dcg: f64,
}

impl SearchResult {
    pub fn evaluate(&self, labels: &Labels) -> EvaluationResult {
        labels.get(&self.query_id).map_or(
            EvaluationResult {
                i_dcg: 0.0,
                dcg: 0.0,
            },
            |label| {
                let mut score = 0.0;
                for (index, document_id) in self.document_ids.iter().enumerate() {
                    let gain = label.get_gain(document_id).map_or(0.0, |g| g);
                    score += 1.0 / log2(index as f64 + 2.0) * gain
                }
                EvaluationResult {
                    i_dcg: label.i_dcg(),
                    dcg: score,
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::label::{IdealResult, Label};

    #[test]
    fn test_search_result_evaluate() {
        let search_result = SearchResult {
            query_id: "query1".to_string(),
            document_ids: vec!["doc1".to_string(), "doc2".to_string(), "doc3".to_string()],
        };
        let labels = Labels::new(vec![Label::new(
            "query1".to_string(),
            vec![
                IdealResult {
                    document_id: "doc2".to_string(),
                    impairment_gain: 0.2,
                },
                IdealResult {
                    document_id: "doc1".to_string(),
                    impairment_gain: 0.5,
                },
                IdealResult {
                    document_id: "doc3".to_string(),
                    impairment_gain: 1.0,
                },
            ],
        )]);

        let actual = search_result.evaluate(&labels);
        assert_eq!(
            actual,
            EvaluationResult {
                i_dcg: 1.0 / log2(2.0) * 1.0 + 1.0 / log2(3.0) * 0.5 + 1.0 / log2(4.0) * 0.2,
                dcg: 1.0 / log2(2.0) * 0.5 + 1.0 / log2(3.0) * 0.2 + 1.0 / log2(4.0) * 1.0
            }
        )
    }

    #[test]
    fn test_search_result_evaluate_none_label() {
        let search_result = SearchResult {
            query_id: "query1".to_string(),
            document_ids: vec!["doc1".to_string(), "doc2".to_string(), "doc3".to_string()],
        };
        let labels = Labels::new(vec![Label::new(
            "query2".to_string(),
            vec![
                IdealResult {
                    document_id: "doc2".to_string(),
                    impairment_gain: 0.2,
                },
                IdealResult {
                    document_id: "doc1".to_string(),
                    impairment_gain: 0.5,
                },
                IdealResult {
                    document_id: "doc3".to_string(),
                    impairment_gain: 1.0,
                },
            ],
        )]);

        let actual = search_result.evaluate(&labels);
        assert_eq!(
            actual,
            EvaluationResult {
                i_dcg: 0.0,
                dcg: 0.0
            }
        )
    }

    #[test]
    fn test_search_result_evaluate_none_gain() {
        let search_result = SearchResult {
            query_id: "query1".to_string(),
            document_ids: vec!["doc1".to_string(), "doc2".to_string(), "doc3".to_string()],
        };
        let labels = Labels::new(vec![Label::new(
            "query1".to_string(),
            vec![
                IdealResult {
                    document_id: "doc4".to_string(),
                    impairment_gain: 0.2,
                },
                IdealResult {
                    document_id: "doc1".to_string(),
                    impairment_gain: 0.5,
                },
                IdealResult {
                    document_id: "doc3".to_string(),
                    impairment_gain: 1.0,
                },
            ],
        )]);

        let actual = search_result.evaluate(&labels);
        assert_eq!(
            actual,
            EvaluationResult {
                i_dcg: 1.0 / log2(2.0) * 1.0 + 1.0 / log2(3.0) * 0.5 + 1.0 / log2(4.0) * 0.2,
                dcg: 1.0 / log2(2.0) * 0.5 + 0.0 + 1.0 / log2(4.0) * 1.0
            }
        )
    }
}
