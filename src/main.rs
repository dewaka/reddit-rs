extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

use hyper::Client;
use hyper::client::Request;
use hyper::header::UserAgent;
use std::io::{self, Write};

use serde_json::{Error, Value};

use futures::Future;
use futures::stream::Stream;

fn parse_json_example() -> Result<(), Error> {
    let data = r#"{
     "name": "John Doe",
     "age": 43,
     "phones": [
       "+44 1234567",
       "+44 2345678"
     ]
    }"#;

    let v: Value = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Photo {
    url: String,
    photo_size: i32,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    post_count: u32,
    likes_burgers: bool,
    avatar: Option<Photo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChildItem {
    title: String,
    ups: i32,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChildData {
    data: ChildItem,
}

#[derive(Serialize, Deserialize, Debug)]
struct RespData {
    children: Vec<ChildData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: RespData,
}

fn json_decoding_example() -> Result<(), Error> {
    let data = r#"[
      { "url": "http://cdewaka.com", "photo_size": 2200, "smile": "yes" }
    , { "url": "http://dewaka.com", "photo_size": 2400 }
    ]"#;

    let rdata = r#"{"data": {
        "children": [
            { "title":"What category type does Rust fall under? (Absolute beginner at programming)",
            "ups":1,
            "url":"https://www.reddit.com/r/rust/comments/7zucnl/what_category_type_does_rust_fall_under_absolute/"
            }
        ]
      }
    }"#;

    let ps: Vec<Photo> = serde_json::from_str(data)?;
    for p in ps {
        println!("Photo size: {}", p.photo_size);
    }

    let d: Response = serde_json::from_str(rdata)?;
    println!("Got data:\n{:?}", d);

    Ok(())
}

fn get_subreddit(sub: &str) {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let sub_reddit = format!("https://www.reddit.com/r/{}.json", sub);
    let url = sub_reddit.parse::<hyper::Uri>().unwrap();
    let mut req = hyper::Request::new(hyper::Method::Get, url);
    req.headers_mut().set(UserAgent::new("Bubba"));

    let work = client.request(req).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let resp_data : Response = serde_json::from_slice(&body)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // titles
            for c in resp_data.data.children {
                println!("[{}] {} - {}", c.data.ups, c.data.title, c.data.url);
            }

            Ok(())
        })
    });

    core.run(work).unwrap();
}

fn main() {
    println!("*** r/rust top submissions ***");
    get_subreddit("rust");
}
