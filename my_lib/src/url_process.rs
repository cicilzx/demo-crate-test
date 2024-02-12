use url::Url;

pub fn process_url(url_str: &str) {
    let x=1;
    match Url::parse(url_str) {
        Ok(url) => {
            println!("Scheme: {}", url.scheme());
            println!("Host: {:?}", url.host());
            println!("Path: {}", url.path());
            println!("Query: {:?}", url.query());
            println!("Fragment: {:?}", url.fragment());
        },
        Err(e) => println!("Failed to parse URL: {}", e),
    }
}