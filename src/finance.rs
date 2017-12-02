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
	hr_low : f64,
	hr_high : f64
}

// Fetch financial data in the form of findata struct for storing
pub fn fetch(api_key : String) -> FinanceData {
	return FinanceData {
		open : 1.0,
		close : 1.0,
		current : 1.0,
		hr_low : 1.0,
		hr_high : 1.0		
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
			
			//time interval wanted
			let series = "Time Series (1min)";
			
			//time of last refreshed
			let mut time = serde_json::to_string(&v["Meta Data"]["3. Last Refreshed"]).unwrap();
			time.remove(0);
			time.pop();
			println!("{}", time);
			
			//open of last refreshed
			let mut open= serde_json::to_string(&v[series][&time]["1. open"]).unwrap();
			open.remove(0);
			open.pop();
			println!("{}", open);
			let openfloat: f64 = open.parse().unwrap();
			println!("{}", openfloat);
			
			//high of last refreshed
			let mut high = serde_json::to_string(&v[series][&time]["2. high"]).unwrap();
			high.remove(0);
			high.pop();
			println!("{}", high);
			let highfloat: f64 = high.parse().unwrap();
			println!("{}", highfloat);
			
			//low of last refreshed
			let mut low = serde_json::to_string(&v[series][&time]["3. low"]).unwrap();
			low.remove(0);
			low.pop();
			println!("{}", low);
			let lowfloat: f64 = low.parse().unwrap();
			println!("{}", openfloat);
			
			//close of last refreshed
			let mut close = serde_json::to_string(&v[series][&time]["4. close"]).unwrap();
			close.remove(0);
			close.pop();
			println!("{}", close);
			let closefloat: f64 = close.parse().unwrap();
			println!("{}", closefloat);
			
			//volume of last refreshed
			let mut volume = serde_json::to_string(&v[series][&time]["5. volume"]).unwrap();
			volume.remove(0);
			volume.pop();
			println!("{}", volume);
			let volumefloat: f64 = volume.parse().unwrap();
			println!("{}", volumefloat);
			
			//print checks
			//println!("{}", v[series][&time]);
			//println!("{}", v[series][&time]["1. open"]);
			//println!("{}", v[series][&time]["2. high"]);
			//println!("{}", v[series][&time]["3. low"]);
			//println!("{}", v[series][&time]["4. close"]);
			//println!("{}", v[series][&time]["5. volume"]);
			
			
			
			
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
