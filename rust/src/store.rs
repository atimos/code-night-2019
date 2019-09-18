use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use std::{fmt::Display, str::FromStr};

#[derive(Deserialize, Serialize, Debug)]
pub struct Store {
    #[serde(rename = "store")]
    pub info: StoreInfo,
    #[serde(rename = "items")]
    pub beers: Beers,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StoreInfo {
    pub id: i32,
    pub address: String,
    pub city: String,
    pub county: String,
    pub lat: String,
    pub lng: String,
}

pub type Beers = Vec<Beer>;

#[derive(Deserialize, Serialize, Debug)]
pub struct Beer {
    pub id: i32,
    pub sysid: i32,
    pub name: String,
    pub country: String,
    #[serde(deserialize_with = "f32_from_str")]
    pub alcohol_vol: f32,
    pub volume_ml: i32,
    #[serde(deserialize_with = "f32_from_str")]
    pub price: f32,
    pub first_sale: String,
}

impl Beer {
    pub fn apk(&self) -> f32 {
        self.alcohol_vol / 100. * (self.volume_ml as f32) / self.price
    }
}

pub fn load(store_id: Option<usize>) -> Result<Store, ()> {
    reqwest::get(&format!(
        "https://systembevakningsagenten.se/api/json/1.0/inventoryForStore.json?id={}",
        store_id.unwrap_or(1337)
    ))
    .map_err(|_| ())?
    .json()
    .map_err(|_| ())
}

fn f32_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
