use libm::log2;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq)]
pub struct Labels {
    map: HashMap<String, Label>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    query_id: String,
    ideal_results: Vec<IdealResult>,
    ideal_result_map: HashMap<String, IdealResult>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdealResult {
    pub document_id: String,
    pub impairment_gain: f64,
}

impl Labels {
    pub fn new(list: Vec<Label>) -> Self {
        Self {
            map: HashMap::from_iter(
                list.into_iter()
                    .map(|label| (label.query_id.clone(), label)),
            ),
        }
    }

    pub fn get(&self, query_id: &str) -> Option<&Label> {
        self.map.get(query_id)
    }
}

impl Label {
    pub fn new(query_id: String, mut ideal_results: Vec<IdealResult>) -> Self {
        ideal_results.sort_by(|left, right| {
            right
                .impairment_gain
                .partial_cmp(&left.impairment_gain)
                .unwrap_or(Ordering::Equal)
        });
        let ideal_result_map = HashMap::from_iter(
            ideal_results
                .iter()
                .map(|ir| (ir.document_id.clone(), ir.clone())),
        );
        Self {
            query_id,
            ideal_results,
            ideal_result_map,
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
    fn test_labels_new() {
        let label1 = Label {
            query_id: "query1".to_string(),
            ideal_results: vec![],
            ideal_result_map: HashMap::new(),
        };
        let label2 = Label {
            query_id: "query2".to_string(),
            ideal_results: vec![],
            ideal_result_map: HashMap::new(),
        };
        let actual = Labels::new(vec![label1.clone(), label2.clone()]);
        assert_eq!(
            actual,
            Labels {
                map: HashMap::from_iter(vec![
                    ("query1".to_string(), label1),
                    ("query2".to_string(), label2)
                ])
            }
        );
    }

    #[test]
    fn test_labels_get() {
        let label1 = Label {
            query_id: "query1".to_string(),
            ideal_results: vec![],
            ideal_result_map: HashMap::new(),
        };
        let label2 = Label {
            query_id: "query2".to_string(),
            ideal_results: vec![],
            ideal_result_map: HashMap::new(),
        };
        let labels = Labels {
            map: HashMap::from_iter(vec![
                ("query1".to_string(), label1),
                ("query2".to_string(), label2.clone()),
            ]),
        };
        let actual = labels.get("query2");
        assert_eq!(actual, Some(&label2));
    }
    #[test]
    fn test_labels_get_none() {
        let label1 = Label {
            query_id: "query1".to_string(),
            ideal_results: vec![],
            ideal_result_map: HashMap::new(),
        };
        let labels = Labels {
            map: HashMap::from_iter(vec![("query1".to_string(), label1)]),
        };
        let actual = labels.get("query2");
        assert_eq!(actual, None);
    }

    #[test]
    fn test_label_new() {
        let ideal_result_1 = IdealResult {
            document_id: "doc_1".to_string(),
            impairment_gain: 0.5,
        };
        let ideal_result_2 = IdealResult {
            document_id: "doc_2".to_string(),
            impairment_gain: 0.2,
        };
        let ideal_result_3 = IdealResult {
            document_id: "doc_3".to_string(),
            impairment_gain: 1.0,
        };
        let ideal_result_4 = IdealResult {
            document_id: "doc_4".to_string(),
            impairment_gain: 0.2,
        };
        let ideal_result_5 = IdealResult {
            document_id: "doc_5".to_string(),
            impairment_gain: 0.5,
        };
        let ideal_result_6 = IdealResult {
            document_id: "doc_6".to_string(),
            impairment_gain: 1.0,
        };
        let actual = Label::new(
            "query1".to_string(),
            vec![
                ideal_result_1.clone(),
                ideal_result_2.clone(),
                ideal_result_3.clone(),
                ideal_result_4.clone(),
                ideal_result_5.clone(),
                ideal_result_6.clone(),
            ],
        );
        assert_eq!(
            actual,
            Label {
                query_id: "query1".to_string(),
                ideal_results: vec![
                    ideal_result_3.clone(),
                    ideal_result_6.clone(),
                    ideal_result_1.clone(),
                    ideal_result_5.clone(),
                    ideal_result_2.clone(),
                    ideal_result_4.clone(),
                ],
                ideal_result_map: HashMap::from_iter(vec![
                    ("doc_1".to_string(), ideal_result_1.clone()),
                    ("doc_2".to_string(), ideal_result_2.clone()),
                    ("doc_3".to_string(), ideal_result_3.clone()),
                    ("doc_4".to_string(), ideal_result_4.clone()),
                    ("doc_5".to_string(), ideal_result_5.clone()),
                    ("doc_6".to_string(), ideal_result_6.clone()),
                ]),
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
            ideal_result_map: HashMap::new(),
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
