//extern crate futures;
//extern crate hyper;
//extern crate tokio_core;

//use std::io::{self, Write};
//use futures::{Future, Stream};
//use hyper::Client;
//use tokio_core::reactor::Core;

//fn main() {
//    let mut core = Core::new()?;
//    let client = Client::new(&core.handle());
//    let uri = "http://web.mta.info/status/serviceStatus.txt".parse()?;
//    let work = client.get(uri).map(|res| {
//       println!("Response: {}", res.status());
//    });
//}

extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate quick_xml;

use std::io::Read;
use hyper::Client;

fn main() {
    println!("=================================");
    let mut xml_resp = String::new();
    get_mta_status(&mut xml_resp);
}

fn get_mta_status(xml: &mut String) {
    let client = Client::new();
    client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send()
        .unwrap()
        .read_to_string(xml)
        .unwrap();
    println!("{}", xml);
}