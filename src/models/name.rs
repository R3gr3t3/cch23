use serde::Deserialize;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Names {
    pub item: Vec<String>,
}