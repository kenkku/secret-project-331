use std::collections::HashMap;

use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use url::Url;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ts_rs")]
use ts_rs::TS;

use crate::prelude::*;

#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "ts_rs", derive(TS))]
pub struct OEmbedResponse {
    pub author_name: String,
    pub author_url: String,
    pub html: String,
    pub provider_name: String,
    pub provider_url: String,
    pub title: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct OEmbedRequest {
    pub url: String,
}

// https://github.com/WordPress/wordpress-develop/blob/master/src/wp-includes/class-wp-oembed.php
pub fn url_to_oembed_endpoint(url: String, base_url: Option<String>) -> UtilResult<Url> {
    let parsed_url = Url::parse(url.as_str())?;
    if let Some(host) = parsed_url.host_str() {
        if host.ends_with("youtu.be") || host.ends_with("youtube.com") {
            return oembed_url_builder(
                "https://www.youtube.com/oembed",
                &format!("url={}&format=json&maxwidth=780&maxheight=440", url),
            );
        }
        if host.ends_with("twitter.com") {
            return oembed_url_builder(
                "https://publish.twitter.com/oembed",
                &format!("url={}&maxwidth=780&maxheight=440", url),
            );
        }
        if host.ends_with("soundcloud.com") {
            return oembed_url_builder(
                "https://soundcloud.com/oembed",
                &format!("url={}&format=json&maxwidth=780&maxheight=440", url),
            );
        }
        if host.ends_with("open.spotify.com") || host.ends_with("play.spotify.com") {
            return oembed_url_builder(
                "https://embed.spotify.com/oembed",
                &format!("url={}&format=json&height=335&width=780", url),
            );
        }
        if host.ends_with("flickr.com") || host.ends_with("flic.kr") {
            return oembed_url_builder(
                "https://www.flickr.com/services/oembed",
                &format!("url={}&format=json&maxwidth=780&maxheight=780", url),
            );
        }
        if host.ends_with("vimeo.com") {
            return oembed_url_builder(
                "https://vimeo.com/api/oembed.json",
                &format!("url={}&maxwidth=780&maxheight=440", url),
            );
        }
        if host.ends_with("menti.com") || host.ends_with("mentimeter.com") {
            return oembed_url_builder(
                &format!(
                    "{}/api/v0/cms/gutenberg/oembed/mentimeter",
                    base_url.unwrap()
                ),
                &format!(
                    "url={}",
                    utf8_percent_encode(url.as_str(), NON_ALPHANUMERIC)
                ),
            );
        }
        if host.ends_with("imgur.com") {
            return oembed_url_builder(
                "https://api.imgur.com/oembed",
                &format!("url={}&maxwidth=780", url),
            );
        }
        if host.ends_with("reddit.com") {
            return oembed_url_builder(
                "https://www.reddit.com/oembed",
                &format!("url={}&format=json", url),
            );
        }
        if host.ends_with("slideshare.net") {
            return oembed_url_builder(
                "https://www.slideshare.net/api/oembed/2",
                &format!("url={}&format=json", url),
            );
        }
        if host.ends_with("ted.com") {
            return oembed_url_builder(
                "https://www.ted.com/services/v1/oembed.json",
                &format!("url={}", url),
            );
        }
        if host.ends_with("tumblr.com") {
            // Old tumblr api, v2 is latest, but WP uses 1.0
            return oembed_url_builder(
                "https://www.tumblr.com/oembed/1.0",
                &format!("url={}&format=json", url),
            );
        }
        Err(UtilError::new(
            UtilErrorType::Other,
            "Link not supported for embedding.".to_string(),
            None,
        ))
    } else {
        Err(UtilError::new(
            UtilErrorType::Other,
            "Failed to parse host from URL.".to_string(),
            None,
        ))
    }
}

pub fn mentimeter_oembed_response_builder(
    url: String,
    base_url: String,
) -> UtilResult<OEmbedResponse> {
    let mut parsed_url = Url::parse(url.as_str()).unwrap();
    // Get the height and title params
    let params: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
    // We want to remove the query params so that the iframe src url doesn't have them
    parsed_url.set_query(None);
    let decoded_title = percent_decode_str(
        params
            .get("title")
            .unwrap_or(&"Mentimeter%20embed".to_string()),
    )
    .decode_utf8()
    .expect("Decoding title or default value for menti embed failed")
    .to_string();

    let response = OEmbedResponse {
        author_name: "Mooc.fi".to_string(),
        author_url: base_url,
        html: format!(
            "<iframe src={} style='width: 99%;' height={:?} title={:?}></iframe>",
            parsed_url,
            params.get("height").unwrap_or(&"500".to_string()),
            decoded_title
        ),
        provider_name: "mentimeter".to_string(),
        provider_url: parsed_url
            .host_str()
            .unwrap_or("https://www.mentimeter.com")
            .to_string(),
        title: decoded_title,
        version: "1.0".to_string(),
    };
    Ok(response)
}

fn oembed_url_builder(url: &str, query_params: &str) -> UtilResult<Url> {
    let mut endpoint_url = Url::parse(url)?;
    endpoint_url.set_query(Some(query_params));
    Ok(endpoint_url)
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;
    #[test]
    fn works_with_youtu_be() {
        assert_eq!(
            url_to_oembed_endpoint("https://youtu.be/dQw4w9WgXcQ".to_string(), None).unwrap(),
            Url::parse(
                "https://www.youtube.com/oembed?url=https://youtu.be/dQw4w9WgXcQ&format=json&maxwidth=780&maxheight=440"
            )
            .unwrap()
        )
    }
    #[test]
    fn works_with_youtube_com() {
        assert_eq!(
            url_to_oembed_endpoint("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(), None)
                .unwrap(),
            Url::parse("https://www.youtube.com/oembed?url=https://www.youtube.com/watch?v=dQw4w9WgXcQ&format=json&maxwidth=780&maxheight=440").unwrap()
        )
    }
    #[test]
    fn works_with_youtube_com_playlist() {
        assert_eq!(
            url_to_oembed_endpoint(
                "https://www.youtube.com/playlist?list=gYLqDMMh1GEbHf-Q1".to_string(), None
            )
            .unwrap(),
            Url::parse("https://www.youtube.com/oembed?url=https://www.youtube.com/playlist?list=gYLqDMMh1GEbHf-Q1&format=json&maxwidth=780&maxheight=440").unwrap()
        )
    }
    #[test]
    fn works_with_open_spotify_com() {
        assert_eq!(url_to_oembed_endpoint(
            "http://open.spotify.com/track/298gs9ATwKlQR".to_string(), None
        )
        .unwrap(),
        Url::parse("https://embed.spotify.com/oembed?url=http://open.spotify.com/track/298gs9ATwKlQR&format=json&height=335&width=780").unwrap())
    }
    #[test]
    fn works_with_flic_kr_com() {
        assert_eq!(
            url_to_oembed_endpoint("https://flic.kr/p/2jJ".to_string(), None).unwrap(),
            Url::parse(
                "https://www.flickr.com/services/oembed?url=https://flic.kr/p/2jJ&format=json&maxwidth=780&maxheight=780"
            ).unwrap()
        )
    }
}
