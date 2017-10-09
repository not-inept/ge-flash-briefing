#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate config;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
use config::File;
use std::collections::HashMap;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
use serde_json::{Value, Error};

enum FeedFormat {
    Alexa
}

#[derive(Serialize)]
struct AlexaItem {
    uid: String,
    updateDate: String,
    titleText: String,
    mainText: String,
    redirectionUrl: String
}

#[get("/")]
fn index() -> &'static str {
    "Webpage Under Construction!"
}

fn get_feed() -> Result<Value, Error> {

    let mut settings = config::Config::default();
    settings
        .merge(File::with_name("conf/sources.toml")).unwrap();
    let sources = settings.get::< HashMap<String, String>>("event_registry").unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = ("http://eventregistry.org/json/event?query=%7B%22%24query%22%3A%7B%22%24and%22%3A%5B%7B%22conceptUri%22%3A%7B%22%24and%22%3A%5B%22http%3A%2F%2Fen.wikipedia.org%2Fwiki%2FGeneral_Electric%22%5D%7D%7D%2C%7B%22lang%22%3A%22eng%22%7D%5D%7D%7D&action=getEvents&resultType=events&eventsSortBy=rel&eventsCount=20&eventsIncludeEventSummary=true&eventsIncludeEventCategories=false&eventsIncludeEventLocation=false&eventsConceptType=org&eventsIncludeSourceTitle=false&eventsIncludeSourceDetails=true&callback=JSON_CALLBACK"
        .to_string() + &"&apiKey=".to_string() + &sources["api_key"]).parse().unwrap();;
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
    // println!("{}",response);
    let v: Value = serde_json::from_str(&response).unwrap();
    Ok(v)
}

fn format_alexa(v: Value) -> Result<String, Error> {
    // println!("Got to formatting.");
    // println!("Received: {}", v);
    let mut formatted_results : Vec<AlexaItem> = Vec::new();
    let results : &Vec<Value> = v["events"]["results"].as_array().unwrap();
    let mut i = 0;
    let ge_id : String = format!("\"{}\"", 312064);
    for res in results {
        // println!("Look at this! {}", res);

        let concepts : &Vec<Value> = res["concepts"].as_array().unwrap();
        for con in concepts {
            let con_id : String = con["id"].to_string();
            if con_id == ge_id && i < 6 {
                let uid : String = String::from("");
                let update_date : String = res["eventDate"].to_string() + &String::from("T00:00:00.0Z");
                let main_text : String = res["summary"]["eng"].to_string();
                let redirection_url : String = String::from("http://eventregistry.org/event/") + &uid;
                let num_articles : String = res["totalArticleCount"].to_string();
                let title_text : String = num_articles + &String::from(" articles discuss") + &res["title"]["eng"].to_string();
                let cur_result = AlexaItem {
                    mainText: main_text,
                    titleText: title_text,
                    uid: uid,
                    updateDate: update_date,
                    redirectionUrl: redirection_url
                };
                formatted_results.push(cur_result);
                i += 1;
            }
        }

    }
    Ok(serde_json::to_string(&formatted_results)?)
}

fn build_feed(format:FeedFormat) -> String {
    let v: Value = get_feed().unwrap();
    match format {
        FeedFormat::Alexa => {
            return format_alexa(v).unwrap();
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
    // println!("{}", build_feed(FeedFormat::Alexa));
    rocket::ignite()
        .mount("/", routes![index, serve_feed])
        .launch();
}
