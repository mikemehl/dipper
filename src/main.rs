mod db;
mod podcast;
mod feed;

fn main() {
    println!("Hello, world!");
    let _ = db::init_db("test.db".to_string());
    let mut curl_handle = feed::init_curl().unwrap();
    let test_rss: String = feed::fetch_rss(&mut curl_handle, "https://www.pipes.digital/feed/7N3mlbqy").unwrap();
    let pod = feed::parse_rss("https://www.pipes.digital/feed/7N3mlbqy", &test_rss).unwrap();
    println!("{}", test_rss);
    println!("{:?}", pod);
}
