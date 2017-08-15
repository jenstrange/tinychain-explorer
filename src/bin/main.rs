extern crate iron;
extern crate handlebars_iron;
extern crate byteorder;

use std::collections::HashMap;
use iron::prelude::*;
use handlebars_iron::{HandlebarsEngine, DirectorySource, Template};

use std::io::prelude::*;
use std::net::TcpStream;

use byteorder::{BigEndian, WriteBytesExt};

// TODO: refactor this all to a library
fn send_msg(msg: String) {
    let mut stream = TcpStream::connect("tinychain.co:9999").unwrap();

    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(msg.len() as u32).unwrap();

    stream.write(&wtr).unwrap();
}

fn explorer(_: &mut Request) -> IronResult<Response> {
    let mut data = HashMap::new();
    data.insert("title", "tinychain");

    //let data = send_msg("{\"_type\":\"GetUTXOsMsg\"}");

    Ok(Response::with((iron::status::Ok, Template::new("index", data))))
}

fn main() {
    let mut chain = Chain::new(explorer);

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    chain.link_after(hbse);

    Iron::new(chain).http("localhost:3000").unwrap();
}

