use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct SearchParams {
    pub query: String,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub id: String,
    pub nameWithSpace: String,
    pub fullNameWithSpace: String,
    pub labelPosition: LabelPosition,
    pub r#type: String,
    pub alias: String,
    pub numTerms: i32,
    pub floor: Floor,
}

#[derive(Serialize)]
pub struct LabelPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize)]
pub struct Floor {
    pub buildingCode: String,
    pub level: String,
}

pub async fn search(Query(_params): Query<SearchParams>) -> Json<Vec<SearchResult>> {
    Json(vec![
        SearchResult {
            id: "d65471a3-1f77-4412-9acd-6079bed4fa84".to_string(),
            nameWithSpace: "CUC 220".to_string(),
            fullNameWithSpace: "Cohon University Center 220".to_string(),
            labelPosition: LabelPosition {
                latitude: 40.44392022644891,
                longitude: -79.94220130436851,
            },
            r#type: "Food".to_string(),
            alias: "Au Bon Pain at Skibo Café   ABP".to_string(),
            numTerms: 6,
            floor: Floor {
                buildingCode: "CUC".to_string(),
                level: "2".to_string(),
            },
        }
    ])
}