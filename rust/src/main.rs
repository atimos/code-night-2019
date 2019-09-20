#![feature(proc_macro_hygiene, decl_macro)]

mod store;

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rocket::{get, routes};
use rocket_contrib::{
    json,
    json::{Json, JsonValue},
};
use serde::Serialize;
use store::Store;

#[derive(Serialize)]
struct BeersInCountry<'a> {
    country: &'a str,
    count: usize,
}

#[derive(Serialize)]
struct BeerApk<'a> {
    apk: f32,
    name: &'a str,
}

impl<'a> From<&'a store::Beer> for BeerApk<'a> {
    fn from(beer: &'a store::Beer) -> Self {
        Self { apk: beer.apk(), name: &beer.name }
    }
}

#[derive(Serialize, Default)]
struct BeerKit<'a> {
    price: f32,
    beers: Vec<BeerApk<'a>>,
}

// Hur många ölsorter finns i butiken Hammarby Sjöstad, Lugnets Allé 26-28
// id: 1337?
#[get("/uppgift/1")]
fn uppgift1(store: &Store) -> Json<usize> {
    Json(store.beers.iter().unique_by(|beer| beer.sysid).count())
}

// Vilket land har flest ölsorter representerade i samma butik?
#[get("/uppgift/2")]
fn uppgift2(store: &Store) -> Json<Option<BeersInCountry>> {
    Json(
        store
            .beers
            .iter()
            .sorted_by_key(|beer| &beer.country)
            .group_by(|beer| &beer.country)
            .into_iter()
            .map(|(country, beers)| (country, beers.count()))
            .max_by_key(|&(_, count)| count)
            .map(|(country, count)| BeersInCountry { country: &country, count }),
    )
}

// Vilken ölsort har bäst APK-värde i butiken av de öl som lanserats i år?
#[get("/uppgift/3")]
fn uppgift3(store: &Store) -> Json<Option<BeerApk>> {
    Json(
        store
            .beers
            .iter()
            .filter(|beer| beer.first_sale.starts_with("2019"))
            .max_by_key(|beer| OrderedFloat(beer.apk()))
            .map(From::from),
    )
}

// Ni vill ta reda på om APK-värde är något att gå efter när man köper öl,
// så ni ska ta fram ett provsmaknings-kit från en butik, med de 3 öl som har
// bäst APK-värde och de 3 som har sämst APK-värde. Vad kostar kitet? Och vilka
// öl blev det?
#[get("/uppgift/4")]
fn uppgift4(store: &Store) -> Json<BeerKit> {
    Json(
        store
            .beers
            .iter()
            .sorted_by_key(|beer| OrderedFloat(beer.apk()))
            .enumerate()
            .filter(|(idx, _)| !(3..=store.beers.len() - 4).contains(idx))
            .map(|(_, beer)| (beer.price, From::from(beer)))
            .fold(BeerKit::default(), |mut result, (price, item)| {
                result.price += price;
                result.beers.push(item);
                result
            }),
    )
}

#[get("/uppgift")]
fn all(store: &Store) -> JsonValue {
    json!({
        "uppgift1": uppgift1(store).into_inner(),
        "uppgift2": uppgift2(store).into_inner(),
        "uppgift3": uppgift3(store).into_inner(),
        "uppgift4": uppgift4(store).into_inner(),
    })
}

fn main() {
    rocket::ignite().mount("/", routes![uppgift1, uppgift2, uppgift3, uppgift4, all]).launch();
}
