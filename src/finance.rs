extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper_tls;

use std::io;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use serde_json::Value;

// A struct for holding financial data-- makes everything easier to work with
#[derive(Debug, Clone, Serialize)]
pub struct FinanceData {
    time: String,
    open: f64,
    close: f64,
    current: f64,
    hr_low: f64,
    hr_high: f64,
    hr_volume: f64,
}
impl Default for FinanceData {
    fn default() -> FinanceData {
        FinanceData {
            time: String::from("0"),
            open: 1.0,
            close: 1.0,
            current: 1.0,
            hr_low: 1.0,
            hr_high: 1.0,
            hr_volume: 1.0,
        }
    }
}
// Fetch financial data in the form of findata struct for storing
pub fn fetch(api_key: String) -> Vec<FinanceData> {
    // Build client for HTTPS b
    let mut core = Core::new().unwrap();
    let client_config: hyper::client::Config<_, _> = ::hyper::Client::configure().connector(
        hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap(),
    );
    let client = client_config.build(&core.handle());

    // The endpoint for our API request to alpha
    let uri = format!("https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=GE&interval=60min&apikey={}", api_key).parse().unwrap();
    let mut fin_results : Vec<FinanceData> = Vec::new();
    // Now let's make the request!
    println!("Defining work.");
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                .unwrap();

            // time interval wanted
            let series = "Time Series (60min)";

            // return all results
            let res =  v[series].as_object().unwrap();
            // Break down API request data into items representing fin data for a period
            for (key, val) in res {
                let mut fin_result = FinanceData { ..Default::default() };
                fin_result.time = key.clone();

                // Populate the findata struct with the appropriate info
                fin_result.open = val["1. open"].as_str().unwrap().parse().unwrap();
                fin_result.hr_high = val["2. high"].as_str().unwrap().parse().unwrap();
                fin_result.hr_low = val["3. low"].as_str().unwrap().parse().unwrap();
                fin_result.close = val["4. close"].as_str().unwrap().parse().unwrap();
                fin_result.hr_volume = val["5. volume"].as_str().unwrap().parse().unwrap();
                
                // Put the findata struct on the stack
                fin_results.push(fin_result);
            }
            println!("Work completing.");
            Ok(fin_results)
        })
    });
    println!("Running work.");
    let c = core.run(work);
    println!("Parsing result.");
    println!("{:?}", c);
    c.unwrap()
}



//res.body().concat2().and_then(move |body| {

//println!("current IP address is {}", v["origin"]);
//Ok(())
//})



//Ok(())
