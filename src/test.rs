use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::clear;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Address {
    pub street: String,
    pub number: u32,
    pub state: String,
    pub zip_code: u64,
}

impl Default for Address {
    fn default() -> Self {
        let rand = nanoid::nanoid!(5);
        let num = rand::thread_rng().gen_range(1..=999);
        Self {
            street: format!("{rand} St."),
            number: num,
            state: "CA".to_string(),
            zip_code: 00000,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExampleData {
    pub id: String,
    pub username: String,
    pub age: u32,
    pub addresses: Vec<Address>,
}

impl Default for ExampleData {
    fn default() -> Self {
        let rand = nanoid::nanoid!(20);
        let num = rand::thread_rng().gen_range(1..=50);
        Self {
            id: rand.clone(),
            username: format!("{rand}_username"),
            age: num,
            addresses: (1..=rand::thread_rng().gen_range(1..=3))
                .map(|_| Address::default())
                .collect(),
        }
    }
}

// clear cache, make dummy data
#[allow(dead_code)]
pub async fn before_each() -> Vec<ExampleData> {
    clear().await;
    (0..=10_000)
        .into_iter()
        .map(|_| ExampleData::default())
        .collect::<Vec<_>>()
}
