use curl;
use crate::podcast;

pub fn init_curl() -> Result<curl::easy::Easy, curl::Error> {
    let mut handle = curl::easy::Easy::new();
    handle.useragent("dipper/0.1.0")?;
    Ok(handle)
}

pub fn fetch_rss(handle: &mut curl::easy::Easy, url: &str) -> Result<String, curl::Error> {
    let mut buf = Vec::new();
    handle.url(url)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    Ok(String::from_utf8(buf).unwrap())
}

pub fn parse_rss(url: &str, rss: &str) -> Result<podcast::Podcast, rss::Error> {
    let channel = rss::Channel::read_from(rss.as_bytes())?;
    let mut podcast = podcast::Podcast::new(
        channel.title().to_string(),
        channel.description().to_string(),
        url.to_string(),
    );
    podcast.link = Some( channel.link().to_string() );

    podcast.language = extract_podfield(channel.language());
    podcast.pub_date = extract_podfield(channel.pub_date());
    podcast.last_build_date = extract_podfield(channel.last_build_date());
    for item in channel.items() {
        podcast.episodes.push(parse_item(item));
    }
    Ok(podcast)
}

fn extract_podfield(field: Option<&str>) -> Option<String> {
    match field {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

fn parse_item(item: &rss::Item) -> podcast::Episode {
    let mut episode = podcast::Episode::new(
        item.title().unwrap().to_string(),
        item.guid().unwrap().value().to_string(),
        item.description().unwrap().to_string(),
    );
    episode.pub_date = extract_podfield(item.pub_date());
    episode.link = extract_podfield(item.link());
    episode.enclosure = match item.enclosure() {
        Some(enc) => Some(podcast::Enclosure {
            url: enc.url().to_string(),
            length: Some(enc.length().to_string()),
            mime_type: Some(enc.mime_type().to_string()),
        }),
        None => None,
    };
    episode
}
