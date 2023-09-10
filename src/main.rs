mod db;
mod feed;
mod podcast;
mod cli;

fn main() {
    cli::parse_args();
}

#[allow(dead_code)]
fn test() {
    println!("Hello, world!");
    let testdb = db::init_db("test.db".to_string()).unwrap();
    let mut curl_handle = feed::init_curl().unwrap();
    let test_rss: String =
        feed::fetch_rss(&mut curl_handle, "https://www.pipes.digital/feed/7N3mlbqy").unwrap();
    let mut pod = feed::parse_rss("https://www.pipes.digital/feed/7N3mlbqy", &test_rss).unwrap();
    println!("{}", test_rss);
    println!("{:?}", pod);
    let _ = db::insert_podcast(&testdb, &mut pod);
    let pod2 = db::fetch_podcast_and_episodes(&testdb, 1).unwrap();
    println!("{:?}", pod2);
    assert_eq!(pod.episodes.len(), pod2.episodes.len());
    println!("Hello, world!");
    let pods = db::fetch_all_podcasts(&testdb).unwrap();
    println!("{:?}", pods);
    let eps = db::fetch_episodes(&testdb, 1).unwrap();
    println!("{:?}", eps);
}
