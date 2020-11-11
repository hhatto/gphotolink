use regex::Regex;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let url = &args[1];

    let resp = ureq::get(url).call();

    if !resp.ok() {
        println!("error: status_code={}", resp.status());
        return
    };

    let body = resp.into_string().unwrap();

    let re = Regex::new(r"(?P<url>https://lh3\.googleusercontent\.com/[a-zA-Z0-9\-_]*)").unwrap();
    let mut result = HashSet::<String>::new();
    let mut max_len_url = "".to_string();

    for line in body.lines() {
        if re.is_match(line) {
            for cap in re.captures_iter(line) {
                let pickup_url = &cap["url"];
                result.insert(pickup_url.to_string());
                if pickup_url.len() > max_len_url.len() {
                    max_len_url = pickup_url.to_string()
                }
            }
        }
    }

    println!("url is {}", max_len_url);
}
