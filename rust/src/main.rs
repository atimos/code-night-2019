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
use std::cmp::Reverse;
use store::Store;

// Hur många ölsorter finns i butiken Hammarby Sjöstad, Lugnets Allé 26-28
// id: 1337?
#[get("/uppgift/1")]
fn uppgift1(store: &Store) -> Result<Json<usize>, ()> {
    Ok(Json(store.beers.iter().unique_by(|beer| beer.sysid).count()))
}

// Vilket land har flest ölsorter representerade i samma butik?
#[derive(Serialize, Debug, Default)]
struct BeersInCountry<'a> {
    country: &'a str,
    count: usize,
}
#[get("/uppgift/2")]
fn uppgift2(store: &Store) -> Result<Option<Json<BeersInCountry>>, ()> {
    Ok(store
        .beers
        .iter()
        .sorted_by_key(|beer| &beer.country)
        .group_by(|beer| &beer.country)
        .into_iter()
        .map(|(country, beers)| (country, beers.count()))
        .sorted_by_key(|&(_, count)| Reverse(count))
        .next()
        .map(|(country, count)| Json(BeersInCountry { country: &country, count })))
}

// Vilken ölsort har bäst APK-värde i butiken av de öl som lanserats i år?
#[derive(Serialize, Debug, Default)]
struct BeerApk<'a> {
    apk: f32,
    beer: &'a str,
}
#[get("/uppgift/3")]
fn uppgift3(store: &Store) -> Result<Option<Json<BeerApk>>, ()> {
    Ok(store
        .beers
        .iter()
        .filter(|beer| beer.first_sale.starts_with("2019"))
        .sorted_by_key(|beer| Reverse(OrderedFloat(beer.apk())))
        .next()
        .map(|beer| Json(BeerApk { apk: beer.apk(), beer: &beer.name })))
}

// Ni vill ta reda på om APK-värde är något att gå efter när man köper öl,
// så ni ska ta fram ett provsmaknings-kit från en butik, med de 3 öl som har
// bäst APK-värde och de 3 som har sämst APK-värde. Vad kostar kitet? Och vilka
// öl blev det?
#[derive(Serialize, Debug, Default)]
struct BeerKit<'a> {
    price: f32,
    beers: Vec<&'a str>,
}
#[get("/uppgift/4")]
fn uppgift4(store: &Store) -> Result<Json<BeerKit>, ()> {
    let first_index_to_skip = 3;
    let last_index_to_skip = &store.beers.len() - 4;

    Ok(Json(
        store
            .beers
            .iter()
            .sorted_by_key(|beer| Reverse(OrderedFloat(beer.apk())))
            .enumerate()
            .filter(|(idx, _)| *idx < first_index_to_skip || *idx > last_index_to_skip)
            .map(|(_, beer)| beer)
            .fold(BeerKit::default(), |mut result, beer| {
                result.price += beer.price;
                result.beers.push(&beer.name);
                result
            }),
    ))
}

#[get("/uppgift")]
fn all(store: &Store) -> Result<JsonValue, ()> {
    Ok(json!({
        "uppgift1": uppgift1(store)?.into_inner(),
        "uppgift2": uppgift2(store)?.map(Json::into_inner),
        "uppgift3": uppgift3(store)?.map(Json::into_inner),
        "uppgift4": uppgift4(store)?.into_inner(),
    }))
}

fn main() {
    rocket::ignite().mount("/", routes![uppgift1, uppgift2, uppgift3, uppgift4, all]).launch();
}
