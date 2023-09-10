mod db;
mod feed;
mod podcast;

fn main() {
    println!("Hello, world!");
    let testdb = db::init_db("test.db".to_string()).unwrap();
    let mut curl_handle = feed::init_curl().unwrap();
    let test_rss: String =
        feed::fetch_rss(&mut curl_handle, "https://www.pipes.digital/feed/7N3mlbqy").unwrap();
    let mut pod = feed::parse_rss("https://www.pipes.digital/feed/7N3mlbqy", &test_rss).unwrap();
    println!("{}", test_rss);
    println!("{:?}", pod);
    let _ = db::insert_podcast(&testdb, &mut pod);
}
