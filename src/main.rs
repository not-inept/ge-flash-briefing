#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

enum FeedFormat {
    Alexa
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn build_feed(format:FeedFormat) -> String {
    match format {
        FeedFormat::Alexa => {
            return String::from("alexa feed");
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
    rocket::ignite().mount("/", routes![index]).launch();
}
