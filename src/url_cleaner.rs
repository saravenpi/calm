use url::Url;
use crate::config::Config;

/// List of known tracking parameter names to remove from URLs.
const TRACKING_PARAMS: &[&str] = &[
    "utm_source",
    "utm_medium",
    "utm_campaign",
    "utm_term",
    "utm_content",
    "utm_cid",
    "utm_internal",
    "utm_klaviyo_id",
    "utm_reader",
    "utm_referrer",
    "utm_name",
    "utm_social",
    "utm_social-type",
    "mtm_source",
    "mtm_medium",
    "mtm_campaign",
    "mtm_keyword",
    "mtm_content",
    "mtm_cid",
    "fbclid",
    "gclid",
    "msclkid",
    "dclid",
    "yclid",
    "srsltid",
    "twclid",
    "gbraid",
    "wbraid",
    "_ga",
    "_gl",
    "ga_source",
    "ga_medium",
    "ga_term",
    "ga_content",
    "ga_campaign",
    "ga_place",
    "mkt_tok",
    "mc_cid",
    "mc_eid",
    "igshid",
    "vgo_ee",
    "_hsenc",
    "_hsmi",
    "__hsfp",
    "__hssc",
    "__hstc",
    "cvid",
    "oicd",
    "oly_anon_id",
    "oly_enc_id",
    "otc",
    "wickedid",
    "rb_clickid",
    "ICID",
    "soc_src",
    "soc_trk",
    "_openstat",
    "vero_id",
    "vero_conv",
    "spm",
    "ref",
    "ref_",
    "affiliate_id",
    "campaign_id",
    "ad_id",
    "source",
];

/// List of tracking parameter prefixes to match against.
const TRACKING_PREFIXES: &[&str] = &[
    "stm_",
    "pk_",
];

/// Checks if a hostname is a YouTube domain.
fn is_youtube_url(host: &str) -> bool {
    matches!(host, "youtube.com" | "www.youtube.com" | "m.youtube.com" | "youtu.be")
}

/// Extracts YouTube video ID from a path given a prefix.
fn extract_video_id_from_path(path: &str, prefix: &str) -> Option<String> {
    path.strip_prefix(prefix)
        .and_then(|s| s.split('/').next())
        .filter(|id| !id.is_empty())
        .map(|id| id.to_string())
}

/// Converts video ID and query parameters to standard YouTube watch URL format.
fn convert_to_watch_url(video_id: &str, query: &str) -> (String, String) {
    let new_query = if query.is_empty() {
        format!("v={}", video_id)
    } else {
        format!("v={}&{}", video_id, query)
    };
    ("/watch".to_string(), new_query)
}

/// Processes YouTube URL paths and converts various formats to standard watch URLs.
fn process_youtube_path(host: &str, path: &str, query: &str) -> (String, String) {
    if host == "youtu.be" {
        if let Some(video_id) = extract_video_id_from_path(path, "/") {
            return convert_to_watch_url(&video_id, query);
        }
    }

    let video_path_prefixes = ["/shorts/", "/embed/", "/v/", "/live/"];
    for prefix in &video_path_prefixes {
        if let Some(video_id) = extract_video_id_from_path(path, prefix) {
            return convert_to_watch_url(&video_id, query);
        }
    }

    (path.to_string(), query.to_string())
}

/// Redirects YouTube URLs to an Invidious instance if enabled in config.
///
/// Handles all YouTube URL formats:
/// - youtube.com/watch?v=VIDEO_ID
/// - youtu.be/VIDEO_ID
/// - youtube.com/embed/VIDEO_ID
/// - youtube.com/v/VIDEO_ID
/// - youtube.com/shorts/VIDEO_ID (converted to regular watch)
/// - youtube.com/live/VIDEO_ID
/// - youtube.com/channel/CHANNEL_ID
/// - youtube.com/c/CHANNEL_NAME
/// - youtube.com/@USERNAME
/// - youtube.com/playlist?list=PLAYLIST_ID
///
/// # Arguments
/// * `url_str` - The URL string to potentially redirect
/// * `config` - The configuration containing redirect settings
///
/// # Returns
/// * `String` - The redirected URL if it's a YouTube URL and redirect is enabled, otherwise the original URL
pub fn redirect_youtube_to_invidious(url_str: &str, config: &Config) -> String {
    if !config.redirect_youtube_to_invidious {
        return url_str.to_string();
    }

    let url = match Url::parse(url_str) {
        Ok(u) => u,
        Err(_) => return url_str.to_string(),
    };

    let host = match url.host_str() {
        Some(h) if is_youtube_url(h) => h,
        _ => return url_str.to_string(),
    };

    let path = url.path();
    let query = url.query().unwrap_or("");

    let (new_path, new_query) = process_youtube_path(host, path, query);

    if new_query.is_empty() {
        format!("https://{}{}", config.invidious_instance, new_path)
    } else {
        format!("https://{}{}?{}", config.invidious_instance, new_path, new_query)
    }
}

/// Removes tracking parameters from a URL string.
/// Returns a cleaned URL with all known tracking parameters removed.
///
/// # Arguments
/// * `url_str` - The URL string to clean
///
/// # Returns
/// * `Ok(String)` - The cleaned URL without tracking parameters
/// * `Err(url::ParseError)` - If the URL cannot be parsed
pub fn clean_url(url_str: &str) -> Result<String, url::ParseError> {
    let mut url = Url::parse(url_str)?;

    let params_to_remove: Vec<String> = url
        .query_pairs()
        .filter_map(|(key, _)| {
            let key_str = key.as_ref();
            if TRACKING_PARAMS.contains(&key_str)
                || TRACKING_PREFIXES.iter().any(|prefix| key_str.starts_with(prefix))
            {
                Some(key.into_owned())
            } else {
                None
            }
        })
        .collect();

    if !params_to_remove.is_empty() {
        let new_query_pairs: Vec<(String, String)> = url
            .query_pairs()
            .filter_map(|(key, value)| {
                let key_owned = key.into_owned();
                if !params_to_remove.contains(&key_owned) {
                    Some((key_owned, value.into_owned()))
                } else {
                    None
                }
            })
            .collect();

        if new_query_pairs.is_empty() {
            url.set_query(None);
        } else {
            let query_string = new_query_pairs
                .iter()
                .map(|(k, v)| {
                    if v.is_empty() {
                        k.clone()
                    } else {
                        format!("{}={}", k, v)
                    }
                })
                .collect::<Vec<_>>()
                .join("&");
            url.set_query(Some(&query_string));
        }
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youtube_redirect_disabled() {
        let config = Config {
            redirect_youtube_to_invidious: false,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, url);
    }

    #[test]
    fn test_youtube_watch_redirect() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/watch?v=dQw4w9WgXcQ");
    }

    #[test]
    fn test_youtu_be_redirect() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://youtu.be/dQw4w9WgXcQ";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/watch?v=dQw4w9WgXcQ");
    }

    #[test]
    fn test_youtube_shorts_redirect() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/shorts/dQw4w9WgXcQ";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/watch?v=dQw4w9WgXcQ");
    }

    #[test]
    fn test_youtube_embed_redirect() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/embed/dQw4w9WgXcQ";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/watch?v=dQw4w9WgXcQ");
    }

    #[test]
    fn test_youtube_live_redirect() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/live/dQw4w9WgXcQ";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/watch?v=dQw4w9WgXcQ");
    }

    #[test]
    fn test_youtube_channel_redirect() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/channel/UC123456789";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/channel/UC123456789");
    }

    #[test]
    fn test_youtube_at_username_redirect() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/@username";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/@username");
    }

    #[test]
    fn test_youtube_with_timestamp() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42s";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/watch?v=dQw4w9WgXcQ&t=42s");
    }

    #[test]
    fn test_youtu_be_with_timestamp() {
        let config = Config {
            redirect_youtube_to_invidious: true,
            invidious_instance: "inv.nadeko.net".to_string(),
            ..Config::default()
        };
        let url = "https://youtu.be/dQw4w9WgXcQ?t=42";
        let result = redirect_youtube_to_invidious(url, &config);
        assert_eq!(result, "https://inv.nadeko.net/watch?v=dQw4w9WgXcQ&t=42");
    }

    #[test]
    fn test_removes_utm_parameters() {
        let url = "https://example.com/page?utm_source=google&utm_medium=cpc&id=123";
        let cleaned = clean_url(url).unwrap();
        assert_eq!(cleaned, "https://example.com/page?id=123");
    }

    #[test]
    fn test_removes_fbclid() {
        let url = "https://example.com/article?fbclid=IwAR123&page=2";
        let cleaned = clean_url(url).unwrap();
        assert_eq!(cleaned, "https://example.com/article?page=2");
    }

    #[test]
    fn test_removes_all_query_params_when_only_tracking() {
        let url = "https://example.com/page?utm_source=twitter&gclid=abc123";
        let cleaned = clean_url(url).unwrap();
        assert_eq!(cleaned, "https://example.com/page");
    }

    #[test]
    fn test_keeps_legitimate_params() {
        let url = "https://example.com/search?q=rust&page=2";
        let cleaned = clean_url(url).unwrap();
        assert_eq!(cleaned, "https://example.com/search?q=rust&page=2");
    }

    #[test]
    fn test_removes_prefix_patterns() {
        let url = "https://example.com/page?stm_campaign=email&pk_source=newsletter&id=5";
        let cleaned = clean_url(url).unwrap();
        assert_eq!(cleaned, "https://example.com/page?id=5");
    }

    #[test]
    fn test_no_query_params() {
        let url = "https://example.com/page";
        let cleaned = clean_url(url).unwrap();
        assert_eq!(cleaned, "https://example.com/page");
    }

    #[test]
    fn test_calmfile_protocol() {
        let url = "calmfile://localhost/Users/test/file.html";
        let result = clean_url(url);
        match result {
            Ok(cleaned) => assert_eq!(cleaned, "calmfile://localhost/Users/test/file.html"),
            Err(e) => panic!("Failed to parse calmfile:// URL: {:?}", e),
        }
    }
}
