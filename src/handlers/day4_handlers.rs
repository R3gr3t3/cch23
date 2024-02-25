use axum::Json;
use serde::Serialize;

use crate::models::reindeer::{Reindeer2, Reindeers1, Reindeers2};

#[derive(Serialize)]
pub struct RetFormat {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}

impl RetFormat {
    pub fn new() -> RetFormat {
        RetFormat {
            fastest: String::new(),
            tallest: String::new(),
            magician: String::new(),
            consumer: String::new(),
        }
    }
}

// fn sort_by_key<T, K, F>(data: &mut Vec<T>, mut key_extractor: F) -> K
// where
//     T: Clone,
//     K: Ord + Clone,
//     F: FnMut(&T) -> K,
// {
//     data.sort_by_key(|item| key_extractor(item));
//     key_extractor(data.last().unwrap()).clone()
// }

fn sort_by_key<U, V>(data: &mut Vec<Reindeer2>, mut key_extractor: U) -> &Reindeer2
where
    U: FnMut(&Reindeer2) -> &V,
    V: PartialOrd,
{
    // data.sort_by_key(key_extractor);
    data.sort_by(|a, b| {
        key_extractor(a)
            .partial_cmp(key_extractor(b))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    data.last().unwrap()
}

pub async fn task1_handler(Json(reindeers): Json<Reindeers1>) -> String {
    let mut result = 0;
    for reindeer in reindeers.item {
        result += reindeer.strength;
    }

    format!("{}", result)
}

pub async fn task2_handler(Json(mut reindeers): Json<Reindeers2>) -> Json<RetFormat> {
    let mut ret = RetFormat::new();

    let tmp = sort_by_key(reindeers.item.as_mut(), |p| &p.speed);
    ret.fastest = format!(
        "Speeding past the finish line with a strength of {} is {}",
        tmp.strength, tmp.name
    );

    let tmp = sort_by_key(reindeers.item.as_mut(), |p| &p.height);
    ret.tallest = format!(
        "{} is standing tall with his {} cm wide antlers",
        tmp.name, tmp.antler_width
    );

    let tmp = sort_by_key(reindeers.item.as_mut(), |p| &p.snow_magic_power);
    ret.magician = format!(
        "{} could blast you away with a snow magic power of {}",
        tmp.name, tmp.snow_magic_power
    );

    let tmp = sort_by_key(reindeers.item.as_mut(), |p| &p.candies_eaten_yesterday);
    ret.consumer = format!(
        "{} ate lots of candies, but also some {}",
        tmp.name, tmp.favorite_food
    );

    Json(ret)
}
