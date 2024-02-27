use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::models::name::{Names};


#[derive(Deserialize)]
pub struct Pagination {
    split: Option<usize>,
    offset: Option<usize>,
    limit: Option<usize>,
}

// pub async fn task1_handler(pagination: Query<Pagination>, Json(names): Json<Names>) -> Json<Vec<String>> {
//     let pagination = pagination.0;

//     if pagination.limit + pagination.offset > names.item.len() {
//         panic!("Out of range!");
//     }

//     Json(names.item[pagination.offset..pagination.offset + pagination.limit].to_vec())
// }

#[derive(Serialize)]
#[serde(untagged)]
pub enum RetFormat {
    WithSplit(Vec<Vec<String>>),
    WithoutSplit(Vec<String>),
}

pub async fn task2_handler(pagination: Query<Pagination>, Json(names): Json<Names>) -> Json<RetFormat> {
    let result;

    if names.item.len() == 0 {
        return Json(RetFormat::WithoutSplit(Vec::new()));
    }

    let offset = match pagination.0.offset {
        Some(offset) => offset,
        None => 0,
    };

    if let Some(limit) = pagination.0.limit {
        result = names.item[offset..offset + limit].to_vec();
    } else {
        result = names.item[offset..].to_vec();
    }

    if let Some(split) = pagination.0.split {
        let result_split: Vec<Vec<String>> = result.chunks(split).map(|chunk| chunk.to_vec()).collect();

        Json(RetFormat::WithSplit(result_split))
    } else {
        Json(RetFormat::WithoutSplit(result))
    }
}