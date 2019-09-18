#![feature(proc_macro_hygiene, decl_macro)]

mod store;

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rocket::{get, routes};
use rocket_contrib::json::Json;
use std::cmp::Reverse;

// Hur många ölsorter finns i butiken Hammarby Sjöstad, Lugnets Allé 26-28
// id: 1337?
#[get("/uppgift/1?<store_id>")]
fn uppgift1(store_id: Option<usize>) -> Result<Json<usize>, ()> {
    Ok(Json(store::load(store_id)?.beers.iter().unique_by(|beer| beer.sysid).count()))
}

// Vilket land har flest ölsorter representerade i samma butik?
#[get("/uppgift/2?<store_id>")]
fn uppgift2(store_id: Option<usize>) -> Result<Option<Json<(String, usize)>>, ()> {
    Ok(store::load(store_id)?
        .beers
        .iter()
        .sorted_by_key(|beer| &beer.country)
        .group_by(|beer| &beer.country)
        .into_iter()
        .map(|(country, beers)| (country, beers.count()))
        .sorted_by_key(|&(_, count)| Reverse(count))
        .next()
        .map(|(country, count)| Json((country.into(), count))))
}

// Vilken ölsort har bäst APK-värde i butiken av de öl som lanserats i år?
#[get("/uppgift/3?<store_id>")]
fn uppgift3(store_id: Option<usize>) -> Result<Option<Json<(f32, String)>>, ()> {
    Ok(store::load(store_id)?
        .beers
        .into_iter()
        .filter(|beer| beer.first_sale.starts_with("2019"))
        .sorted_by_key(|beer| Reverse(OrderedFloat(beer.apk())))
        .next()
        .map(|beer| Json((beer.apk(), beer.name))))
}

// Ni vill ta reda på om APK-värde är något att gå efter när man köper öl,
// så ni ska ta fram ett provsmaknings-kit från en butik, med de 3 öl som har
// bäst APK-värde och de 3 som har sämst APK-värde. Vad kostar kitet? Och vilka
// öl blev det?
#[get("/uppgift/4?<store_id>")]
fn uppgift4(store_id: Option<usize>) -> Result<Json<(f32, Vec<String>)>, ()> {
    let beers = store::load(store_id)?.beers;
    let first_index_to_skip = 3;
    let last_index_to_skip = beers.len() - 4;

    Ok(Json(
        beers
            .into_iter()
            .sorted_by_key(|beer| Reverse(OrderedFloat(beer.apk())))
            .enumerate()
            .filter(|(idx, _)| *idx < first_index_to_skip || *idx > last_index_to_skip)
            .fold((0., Vec::new()), |mut result, (_, beer)| {
                result.0 += beer.price;
                result.1.push(beer.name);
                result
            }),
    ))
}

fn main() {
    rocket::ignite().mount("/", routes![uppgift1, uppgift2, uppgift3, uppgift4]).launch();
}
