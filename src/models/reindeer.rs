use serde::Deserialize;

#[derive(Deserialize)]
pub struct Reindeer1 {
    pub name: String,
    pub strength: i32,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Reindeers1 {
    pub item: Vec<Reindeer1>,
}

#[derive(Deserialize)]
pub struct Reindeer2 {
    pub name: String,
    pub strength: i32,
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favorite_food: String,

    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: i32,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Reindeers2 {
    pub item: Vec<Reindeer2>
}