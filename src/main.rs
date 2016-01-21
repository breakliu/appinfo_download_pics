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
        // read file content
        let p = path.unwrap().path();
        let f = File::open(&p);
        let mut s = String::new();
        f.unwrap().read_to_string(&mut s);

        let file_name = format!("{}", p.display());
        let v: Vec<&str> = file_name.split("/").collect();
        let v: Vec<&str> = v[v.len()-1].split(".").collect();
        let file_name = v[0];

        let re_imglist = Regex::new("imgList.*:.*\\[(?P<imglist>.*)\\]").unwrap();
        for caps in re_imglist.captures_iter(&s) {
            let imglist = caps.name("imglist").unwrap();
            let v: Vec<&str> = imglist.split(",").collect();
            let mut pic_idx = 0;
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

                let save_file_name= format!("./pics/{}_{}.jpg", file_name, pic_idx);
                let mut file = match File::create(&save_file_name) {
                    Err(why) => panic!("couldn't create {}", save_file_name),
                    Ok(file) => file,
                };
                file.write_all(&buffer);

                println!("save file: {}", save_file_name);

                pic_idx += 1;
            }
        }
    }
    println!("{}", count);

    return;
}
