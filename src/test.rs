use rand::Rng;
use serde::{Deserialize, Serialize};

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

pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}
