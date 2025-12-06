use url::Url;

// pub fn get_webpage_url(url: &str) -> Option<Url> {
//     let output = Command::new("yt-dlp")
//         .arg(url)
//         .arg("--print")
//         .arg("webpage_url")
//         .arg("--cookies-from-browser")
//         .arg("firefox")
//         .output()
//         .unwrap()
//         .stdout;
//     match String::from_utf8(output) {
//         Ok(url) => match Url::parse(url.trim()) {
//             Ok(url) => Some(url),
//             Err(e) => {
//                 println!("Invalid url after yt-dlp: {}", url.trim());
//                 println!("Error: {}", e);
//                 None
//             }
//         },
//         Err(_) => None,
//     }
// }

// pub fn get_format(url: &Url, res: Resolution, ctype: ContentType) -> String {
//     match url.host_str() {
//         Some(host) => match host {
//             "www.youtube.com" | "youtube.com" | "youtu.be" => {
//                 get_format_from_res(Host::Youtube, res, ctype)
//             }
//             "www.twitch.tv" => get_format_from_res(Host::Twitch, res, ctype),
//             "holodex.net" => match get_webpage_url(url.as_ref()) {
//                 Some(url) => get_format(&url, res, ctype),
//                 None => panic!("Invalid url: {}", url),
//             },
//             _ => res.unwrap_custom(),
//         },
//         None => panic!("Invalid url: {}", url),
//     }
// }

// pub fn get_list_formats(url: &Url) {
//     let output = Command::new("yt-dlp")
//         .arg(url.as_str())
//         .arg("--print")
//         .arg("formats_table")
//         .arg("--cookies-from-browser")
//         .arg("firefox")
//         .output()
//         .unwrap();
//     if output.status.success() {
//         let formats_table = String::from_utf8(output.stdout).unwrap();
//         println!("{}", formats_table);
//     } else {
//         eprintln!("Failed to get formats table");
//         eprintln!("{}", String::from_utf8(output.stderr).unwrap());
//     }
// }

pub fn _extract_youtube_id(input: &str) -> Option<String> {
    // Normalize the input - add protocol if missing
    let normalized = if input.starts_with("//") {
        format!("https:{}", input)
    } else if !input.starts_with("http://") && !input.starts_with("https://") {
        format!("https://{}", input)
    } else {
        input.to_string()
    };

    let url = Url::parse(&normalized).ok()?;
    extract_youtube_id_from_url(&url)
}

/// Extracts a YouTube video ID from a parsed url::Url struct
pub fn extract_youtube_id_from_url(url: &Url) -> Option<String> {
    let host = url.host_str()?;

    // Check if it's a YouTube domain
    if !is_youtube_domain(host) {
        return None;
    }

    // Try to extract from query parameters (v or vi)
    for (key, value) in url.query_pairs() {
        if key == "v" || key == "vi" {
            let id = value.to_string();
            if is_valid_video_id(&id) {
                return Some(id);
            }
        }
    }

    // Handle youtu.be short URLs
    if host.contains("youtu.be") {
        if let Some(mut segments) = url.path_segments() {
            if let Some(id) = segments.next() {
                if is_valid_video_id(id) {
                    return Some(id.to_string());
                }
            }
        }
    }

    // Handle path-based URLs
    let path = url.path();

    // Try common path patterns: /embed/, /v/, /vi/, /shorts/, /live/
    for prefix in &["/embed/", "/v/", "/vi/", "/shorts/", "/live/"] {
        if let Some(remainder) = path.strip_prefix(prefix) {
            let id = remainder.split(&['/', '?', '#'][..]).next()?;
            if is_valid_video_id(id) {
                return Some(id.to_string());
            }
        }
    }

    // Handle user pages with fragment like #p/u/1/VIDEO_ID or #p/a/u/2/VIDEO_ID
    if let Some(fragment) = url.fragment() {
        if fragment.starts_with("p/") {
            // Split by / and get the last segment, then clean it from query params
            let parts: Vec<&str> = fragment.split('/').collect();
            if let Some(last) = parts.last() {
                // Remove any query parameters that might be in the fragment
                let clean_id = last.split('?').next().unwrap_or(last);
                if is_valid_video_id(clean_id) {
                    return Some(clean_id.to_string());
                }
            }
        }
    }

    None
}

fn is_youtube_domain(host: &str) -> bool {
    host == "youtube.com"
        || host == "www.youtube.com"
        || host == "m.youtube.com"
        || host == "youtube-nocookie.com"
        || host == "www.youtube-nocookie.com"
        || host == "youtu.be"
        || host == "www.youtu.be"
}

fn is_valid_video_id(id: &str) -> bool {
    // YouTube video IDs are typically 11 characters long
    // and contain alphanumeric characters, hyphens, and underscores
    id.len() == 11
        && id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

// #[cfg(test)]
// mod tests {
//     use std::fs;

//     #[test]
//     fn test_aliases() {
//         let aliases_str = fs::read_to_string("/home/alimulap/void/aliases.toml").unwrap();
//         let aliases = toml::from_str::<toml::Value>(&aliases_str).unwrap();
//         assert!(aliases.get("sora").is_some());
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_url_formats() {
        #[rustfmt::skip]
        let test_cases = vec![
            ("https://youtube.com/shorts/dQw4w9WgXcQ?feature=share", "dQw4w9WgXcQ"),
            ("//www.youtube-nocookie.com/embed/up_lNV-yoK4?rel=0", "up_lNV-yoK4"),
            ("http://www.youtube.com/user/Scobleizer#p/u/1/1p3vcRhsYGo", "1p3vcRhsYGo"),
            ("http://www.youtube.com/watch?v=cKZDdG9FTKY&feature=channel", "cKZDdG9FTKY"),
            ("http://www.youtube.com/watch?v=yZ-K7nCVnBI&playnext_from=TL&videos=osPknwzXEas&feature=sub", "yZ-K7nCVnBI"),
            ("http://www.youtube.com/ytscreeningroom?v=NRHVzbJVx8I", "NRHVzbJVx8I"),
            ("http://www.youtube.com/user/SilkRoadTheatre#p/a/u/2/6dwqZw0j_jY", "6dwqZw0j_jY"),
            ("http://youtu.be/6dwqZw0j_jY", "6dwqZw0j_jY"),
            ("http://www.youtube.com/watch?v=6dwqZw0j_jY&feature=youtu.be", "6dwqZw0j_jY"),
            ("http://youtu.be/afa-5HQHiAs", "afa-5HQHiAs"),
            ("http://www.youtube.com/user/Scobleizer#p/u/1/1p3vcRhsYGo?rel=0", "1p3vcRhsYGo"),
            ("http://www.youtube.com/embed/nas1rJpm7wY?rel=0", "nas1rJpm7wY"),
            ("http://www.youtube.com/watch?v=peFZbP64dsU", "peFZbP64dsU"),
            ("http://youtube.com/v/dQw4w9WgXcQ?feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("http://youtube.com/vi/dQw4w9WgXcQ?feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("http://youtube.com/?v=dQw4w9WgXcQ&feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("http://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("http://youtube.com/?vi=dQw4w9WgXcQ&feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("http://youtube.com/watch?v=dQw4w9WgXcQ&feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("http://youtube.com/watch?vi=dQw4w9WgXcQ&feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("http://youtu.be/dQw4w9WgXcQ?feature=youtube_gdata_player", "dQw4w9WgXcQ"),
            ("https://www.youtube.com/live/dQw4w9WgXcQ", "dQw4w9WgXcQ"),
            ("https://www.youtube.com/live/dQw4w9WgXcQ?feature=share", "dQw4w9WgXcQ"),
        ];

        for (url, expected_id) in test_cases {
            let result = _extract_youtube_id(url);
            assert_eq!(
                result,
                Some(expected_id.to_string()),
                "Failed to extract ID from: {}",
                url
            );
        }
    }

    #[test]
    fn test_parsed_url_struct() {
        let url = Url::parse("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();
        assert_eq!(
            _extract_youtube_id(&url.as_str()),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn test_invalid_url() {
        let url = "https://www.example.com/watch?v=dQw4w9WgXcQ";
        assert_eq!(_extract_youtube_id(url), None);
    }
}
