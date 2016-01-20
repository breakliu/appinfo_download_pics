extern crate hyper;
extern crate regex;

use std::io::Read;
use std::io::Write;
use std::fs;
use std::fs::File;

use hyper::Client;
use hyper::header::Connection;

use regex::Regex;

fn main() {
    let paths = fs::read_dir("./appinfo").unwrap();

    let mut count = 0;
    for path in paths {
        let p = path.unwrap().path();
        let f = File::open(&p);
        let mut s = String::new();
        f.unwrap().read_to_string(&mut s);

        println!("{}", p.display());

        let re_imglist = Regex::new("imgList.*:.*\\[(?P<imglist>.*)\\]").unwrap();
        //let re_url = Regex::new("\"(?P<url>.*)\"").unwrap();
        for caps in re_imglist.captures_iter(&s) {
            let imglist = caps.name("imglist").unwrap();
            //println!("{}", imglist);
            let v: Vec<&str> = imglist.split(",").collect();
            for a in v {
                let mut img_url = a;
                img_url = img_url.trim_matches('"');
                println!("{}", img_url);
                count += 1;

                // Create a client.
                let client = Client::new();

                // Creating an outgoing request.
                let mut res = client.get(img_url)
                    // set a header
                    .header(Connection::close())
                    // let 'er go!
                    .send().unwrap();

                // Read the Response.
                let mut buffer = Vec::new();
                res.read_to_end(&mut buffer).unwrap();

                let mut file = match File::create("./pics/foo.jpg") {
                    Err(why) => panic!("couldn't create {}", "./foo.jpg"),
                    Ok(file) => file,
                };
                file.write_all(&buffer);

                //println!("Response: {:b}", body);
                return;
            }
        }
    }
    println!("{}", count);

    return;
}
