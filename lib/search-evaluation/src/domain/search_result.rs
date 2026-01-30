use crate::domain::label::Labels;

#[derive(Debug, PartialEq)]
pub struct SearchResults {
    pub list: Vec<SearchResult>,
}

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub query_id: String,
    pub document_ids: Vec<String>,
}

pub struct EvaluationResult {
    pub label_score: f64,
    pub search_score: f64,
}

impl SearchResult {
    pub fn score(&self, labels: &Labels) -> EvaluationResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_score() {
        let search_result = SearchResult {
            query_id: "query1".to_string(),
            document_ids: vec!["doc1".to_string(), "doc2".to_string()],
        };
    }
}
