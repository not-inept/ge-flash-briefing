#![feature(plugin)]
#![plugin(rocket_codegen)]

// External crates
extern crate rocket;
extern crate config;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate sentiment;
extern crate chrono;

// Internal modules
mod event;
mod finance;
mod analyzer;

use chrono::Local;
use config::File;
use std::collections::HashMap;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use rocket::http::ContentType;
use rocket::response::Response;
use rocket::response::content;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
use serde_json::{Value, Error};

enum FeedFormat {
    Alexa,
    Web
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct AlexaItem {
    uid: String,
    updateDate: String,
    titleText: String,
    mainText: String,
    redirectionUrl: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct WebItem {
    uid: String,
    updateDate: String,
    titleText: String,
    mainText: String,
    redirectionUrl: String,
    sentiment: String,
    numArticles: String
}

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use std::io;
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("webui/build/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("webui/build/").join(file)).ok()
}

fn get_feed() -> Result<Value, Error> {

    let mut settings = config::Config::default();
    settings
        .merge(File::with_name("conf/sources.toml"))
        .unwrap();
    let sources = settings
        .get::<HashMap<String, String>>("event_registry")
        .unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = ("http://eventregistry.org/json/event?query=%7B%22%24query%22%3A%7B%22%24and%22%3A%5B%7B%22conceptUri%22%3A%7B%22%24and%22%3A%5B%22http%3A%2F%2Fen.wikipedia.org%2Fwiki%2FGeneral_Electric%22%5D%7D%7D%2C%7B%22lang%22%3A%22eng%22%7D%5D%7D%7D&action=getEvents&resultType=events&eventsSortBy=rel&eventsCount=20&eventsIncludeEventSummary=true&eventsIncludeEventCategories=false&eventsIncludeEventLocation=false&eventsConceptType=org&eventsIncludeSourceTitle=false&eventsIncludeSourceDetails=true&callback=JSON_CALLBACK"
        .to_string() + &"&apiKey=".to_string() + &sources["api_key"]).parse().unwrap();;
    // let uri = "http://httpbin.org/ip"
    let work = client.get(uri).and_then(|res| res.body().concat2());
    let data = core.run(work).unwrap();

    let mut response = String::from_utf8(data.to_vec()).unwrap();
    response.pop();
    let prefix = "JSON_CALLBACK(";
    for c in prefix.chars() {
        response = String::from(response.trim_left_matches(c));
    }
    // println!("{}",response);
    let v: Value = serde_json::from_str(&response).unwrap();
    Ok(v)
}

fn get_string(s1: String) -> String {
    let s2 = s1.trim_left_matches("\"");
    let s3: String = String::from(s2.trim_right_matches("\""));
    return s3;
}

fn get_sentiment(text: String) -> String {
    let sent = sentiment::analyze(text);
    if sent.negative.score > sent.positive.score {
        return String::from("negative");
    } else if sent.positive.score > sent.negative.score {
        return String::from("positive");
    } else {
        return String::from("neutral");
    }
}

fn format_web(v: Value) -> Result<String, Error> {
    // println!("Got to formatting.");
    // println!("Received: {}", v);
    let mut formatted_results: Vec<WebItem> = Vec::new();
    let results: &Vec<Value> = v["events"]["results"].as_array().unwrap();
    let mut i = 0;
    let ge_id: String = format!("\"{}\"", 312064);
    for res in results {
        // println!("Look at this! {}", res);

        let concepts: &Vec<Value> = res["concepts"].as_array().unwrap();
        for con in concepts {
            let con_id: String = con["id"].to_string();
            if con_id == ge_id && i < 100 {
                let date = Local::now();
                let time = date.format("%s");
                let uid: String = format!("item_num_{}_{}", time, i);
                // let temp_title = format!("GE News as of {}.", date.format("%A %B %e, %Y"));
                // yyyy-MM-dd'T'HH:mm:ss'.0Z'
                let update_date: String = get_string(res["eventDate"].to_string()) +
                    &String::from("T00:00:00.0Z");
                let main_text: String = get_string(res["summary"]["eng"].to_string());
                let sent = get_string(get_sentiment(main_text.clone()));
                let redirection_url: String = String::from("http://eventregistry.org/event/") +
                    &uid;
                let num_articles: String = get_string(res["totalArticleCount"].to_string());
                let title_text: String = num_articles.clone() + &String::from(" articles discuss ") +
                    &get_string(res["title"]["eng"].to_string()) +
                    &String::from(". This is perceived as generally ") +
                    &sent.clone();
                let cur_result = WebItem {
                    mainText: main_text,
                    titleText: title_text,
                    uid: uid,
                    updateDate: update_date,
                    redirectionUrl: redirection_url,
                    sentiment: sent,
                    numArticles: num_articles
                };
                formatted_results.push(cur_result);
                i += 1;
            }
        }

    }
    Ok(serde_json::to_string(&formatted_results)?)
}

fn format_alexa(v: Value) -> Result<String, Error> {
    // println!("Got to formatting.");
    // println!("Received: {}", v);
    let mut formatted_results: Vec<AlexaItem> = Vec::new();
    let results: &Vec<Value> = v["events"]["results"].as_array().unwrap();
    let mut i = 0;
    let ge_id: String = format!("\"{}\"", 312064);
    for res in results {
        // println!("Look at this! {}", res);

        let concepts: &Vec<Value> = res["concepts"].as_array().unwrap();
        for con in concepts {
            let con_id: String = con["id"].to_string();
            if con_id == ge_id && i < 6 {
                let date = Local::now();
                let time = date.format("%s");
                let uid: String = format!("item_num_{}_{}", time, i);
                let temp_title = format!("GE News as of {}.", date.format("%A %B %e, %Y"));
                // yyyy-MM-dd'T'HH:mm:ss'.0Z'
                // let update_date: String = get_string(res["eventDate"].to_string()) +
                //     &String::from("T00:00:00.0Z");
                let main_text: String = get_string(res["summary"]["eng"].to_string());
                let sent = get_sentiment(main_text.clone());
                let redirection_url: String = String::from("http://eventregistry.org/event/") +
                    &uid;
                let num_articles: String = get_string(res["totalArticleCount"].to_string());
                let title_text: String = num_articles + &String::from(" articles discuss ") +
                    &get_string(res["title"]["eng"].to_string()) +
                    &String::from(". This is perceived as generally ") +
                    &get_string(sent);
                let cur_result = AlexaItem {
                    mainText: title_text,
                    titleText: temp_title,
                    uid: uid,
                    updateDate: format!("{}", date.format("%Y-%m-%dT00:00:00.0Z")),
                    redirectionUrl: redirection_url,
                };
                formatted_results.push(cur_result);
                i += 1;
            }
        }

    }
    Ok(serde_json::to_string(&formatted_results)?)
}

fn build_feed(format: FeedFormat) -> String {
    let v: Value = get_feed().unwrap();
    match format {
        FeedFormat::Alexa => {
            return format_alexa(v).unwrap();
        }
        FeedFormat::Web => {
            return format_web(v).unwrap();
        }
    }
}

#[get("/feed/alexa.json")]
fn serve_feed_alexa() -> content::Json<String> {
    return content::Json(build_feed(FeedFormat::Alexa));
}

#[get("/feed/web.json")]
fn serve_feed_web() -> content::Json<String> {
    return content::Json(build_feed(FeedFormat::Web));
}

#[get("/feed/finance.json")]
fn serve_finance() -> content::Json<String> {
    let mut settings = config::Config::default();
    settings
        .merge(File::with_name("conf/sources.toml"))
        .unwrap();
    let alpha_key = settings
        .get::<HashMap<String, String>>("alpha")
        .unwrap()
        .get("api_key")
        .unwrap()
        .clone();
    let fin = finance::fetch(alpha_key);
    return content::Json(serde_json::to_string(&fin).unwrap());
}

fn main() {
    // println!("{}", build_feed(FeedFormat::Alexa));
    // event::fetch();

    // let mut settings = config::Config::default();
    // settings
    //     .merge(File::with_name("conf/sources.toml"))
    //     .unwrap();
    // let alpha_key = settings
    //     .get::<HashMap<String, String>>("alpha")
    //     .unwrap()
    //     .get("api_key")
    //     .unwrap()
    //     .clone();

    // finance::fetch(alpha_key);

    // analyzer::analyze();

    rocket::ignite()
        .mount("/", routes![serve_feed_alexa, serve_feed_web, serve_finance, files, index])
        .launch();
}
