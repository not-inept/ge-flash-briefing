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


fn main(){
	let mut core = Core::new().unwrap();
	 let client = ::hyper::Client::configure()
        .connector(::hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

	let uri = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=GE&interval=1min&apikey=KPMZYNOVSB3KWEIJ".parse().unwrap();
	let work = client.get(uri).and_then(|res| {
		println!("Response: {}", res.status());
		res.body().concat2()
	
	});
	core.run(work).unwrap();

}

//api key
//KPMZYNOVSB3KWEIJ
//https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=GE&interval=1min&apikey=KPMZYNOVSB3KWEIJ




	//res.body().concat2().and_then(move |body| {
		//	let v: Value = serde_json::from_slice(&body).map_err(|e| {
			//	io::Error::new(
				//	io::ErrorKind::Other,
					//e
				//)
			//}).unwrap();
		//	println!("current IP address is {}", v["origin"]);
		//	Ok(())
		//})
