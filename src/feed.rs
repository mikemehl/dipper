use crate::podcast;
use bytes::Bytes;
use chrono::DateTime;

pub fn fetch_rss(url: &str) -> reqwest::Result<String> {
    reqwest::blocking::get(url)?.text()
}

pub fn parse_rss(url: &str, rss: &str) -> Result<podcast::Podcast, rss::Error> {
    let channel = rss::Channel::read_from(rss.as_bytes())?;
    let mut podcast = podcast::Podcast::new(
        channel.title().to_string(),
        channel.description().to_string(),
        url.to_string(),
    );
    podcast.link = Some(channel.link().to_string());

    podcast.language = extract_podfield(channel.language());
    podcast.pub_date = fix_date(channel.pub_date());
    podcast.last_build_date = fix_date(channel.last_build_date());
    for item in channel.items() {
        if let Ok(item) = parse_item(item) {
            podcast.episodes.push(item);
        }
    }
    Ok(podcast)
}

pub fn fetch_enclosure(enclosure: &podcast::Enclosure) -> reqwest::Result<Bytes> {
    reqwest::blocking::get(&enclosure.url)?.bytes()
}

fn extract_podfield(field: Option<&str>) -> Option<String> {
    field.map(|s| s.to_string())
}

fn parse_item(item: &rss::Item) -> Result<podcast::Episode, ()> {
    if item.guid().is_none() {
        return Err(());
    }
    if item.title().is_none() {
        return Err(());
    }
    if item.description().is_none() {
        return Err(());
    }
    let mut episode = podcast::Episode::new(
        item.title().unwrap().to_string(),
        item.guid().unwrap().value().to_string(),
        item.description().unwrap_or_default().to_string(),
    );
    episode.pub_date = fix_date(item.pub_date());
    episode.link = extract_podfield(item.link());
    episode.enclosure = item.enclosure().map(|enc| podcast::Enclosure {
        url: enc.url().to_string(),
        length: Some(enc.length().to_string()),
        mime_type: Some(enc.mime_type().to_string()),
    });
    Ok(episode)
}

fn fix_date(date: Option<&str>) -> Option<String> {
    match date {
        Some(d) => {
            let dt = DateTime::parse_from_rfc2822(d).unwrap();
            Some(dt.format("%Y-%m-%d").to_string())
        }
        None => None,
    }
}
