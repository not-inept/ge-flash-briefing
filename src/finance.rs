extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
extern crate hyper_tls;

use std::io;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json::Value;

pub struct FinanceData {
	open : f64,
	close : f64,
	current : f64,
	24hr_low : f64,
	24hr_high : f64
}

// Fetch financial data in the form of findata struct for storing
pub fn fetch(api_key : String) -> FinanceData {
	return FinanceData {
		open : 1.0,
		close : 1.0,
		current : 1.0,
		24hr_low : 1.0,
		24hr_high : 1.0		
	}
}

fn main(){
	let mut core = Core::new().unwrap();
	 let client = ::hyper::Client::configure()
        .connector(::hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

	let uri = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=GE&interval=1min&apikey=KPMZYNOVSB3KWEIJ".parse().unwrap();
	
	let work = client.get(uri).and_then(|res| {
		println!("Response: {}", res.status());
		res.body().concat2().and_then(move |body| {
			let v: Value = serde_json::from_slice(&body).map_err(|e| {
				io::Error::new(
					io::ErrorKind::Other,
					e
				)
			}).unwrap();
			
			let series = "Time Series (1min)";
			let mut time = serde_json::to_string(&v["Meta Data"]["3. Last Refreshed"]).unwrap();
			time.remove(0);
			time.pop();
			println!("{}", time);
			
			println!("{}", v[series][time]);
			Ok(())
		})
	
	});
	core.run(work).unwrap();


//api key
//KPMZYNOVSB3KWEIJ
//https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=GE&interval=1min&apikey=KPMZYNOVSB3KWEIJ

}



	//res.body().concat2().and_then(move |body| {

		//println!("current IP address is {}", v["origin"]);
		//Ok(())
		//})
		
		

//Ok(())
