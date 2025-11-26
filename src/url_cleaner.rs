use url::Url;

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
