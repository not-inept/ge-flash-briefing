#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate config;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;

use std::io::Read;
use config::File;
use std::collections::HashMap;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::client::Response;
use serde_json::{Value, Error};

enum FeedFormat {
    Alexa
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn get_feed() -> Result<Value, Error> {


    let mut settings = config::Config::default();
    settings
        .merge(File::with_name("conf/sources.toml")).unwrap();
    let sources = settings.get::< HashMap<String, String>>("event_registry").unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = ("http://eventregistry.org/json/article?query=%7B%22%24query%22%3A%7B%22%24and%22%3A%5B%7B%22keyword%22%3A%7B%22%24and%22%3A%5B%22General%22%2C%22Electric%22%5D%7D%7D%5D%7D%7D&action=getArticles&resultType=articles&articlesSortBy=date&articlesCount=20&callback=JSON_CALLBACK".to_string() + &"&apiKey=".to_string() + &sources["api_key"]).parse().unwrap();;
    // let uri = "http://httpbin.org/ip"
    let work = client
        .get(uri)
        .and_then(|res| {
            res.body().concat2()
        });
    let data = core.run(work).unwrap();

    let mut response = String::from_utf8(data.to_vec()).unwrap();
    response.pop();
    let prefix = "JSON_CALLBACK(";
    for c in prefix.chars() {
        response = String::from(response.trim_left_matches(c));
    }
    println!("{}",response);
    let v: Value = serde_json::from_str(&response).unwrap();
    Ok(v)
}

fn format_alexa(v: Value) -> String {
    println!("Got to formatting.");
    return String::from("formatted content");
}

fn build_feed(format:FeedFormat) -> String {
    let v: Value = get_feed().unwrap();
    match format {
        FeedFormat::Alexa => {
            return format_alexa(v);
        }
    }
}

#[get("/feed/<file>")]
fn serve_feed(file: String) -> String {
    if file == String::from("alexa.json") {
        return build_feed(FeedFormat::Alexa);
    } else {
        return String::from("{ 'response': 'Invalid requst.' }");
    }
}

fn main() {
    build_feed(FeedFormat::Alexa);
    // rocket::ignite().mount("/", routes![index]).launch();
}
