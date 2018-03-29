extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let uri = "http://web.mta.info/status/serviceStatus.txt".parse()?;
    let work = client.get(uri).map(|res| {
        println!("Response: {}", res.status());
    });
}
