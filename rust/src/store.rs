use reqwest::{get, Result as ReqwestResult};
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::{fmt::Display, str::FromStr};

#[derive(Deserialize)]
pub struct Store {
    #[serde(rename = "store")]
    pub info: StoreInfo,
    #[serde(rename = "items")]
    pub beers: Vec<Beer>,
}

impl<'a, 'r> FromRequest<'a, 'r> for &'a Store {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.local_cache(load) {
            Ok(store) => Outcome::Success(store),
            Err(_) => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}

#[derive(Deserialize)]
pub struct StoreInfo {
    pub id: i32,
    pub address: String,
    pub city: String,
    pub county: String,
    pub lat: String,
    pub lng: String,
}

#[derive(Deserialize)]
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

fn load() -> ReqwestResult<Store> {
    get("https://systembevakningsagenten.se/api/json/1.0/inventoryForStore.json?id=1337")?.json()
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
