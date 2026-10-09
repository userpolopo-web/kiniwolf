use url::Url;

pub fn normalize_navigation_input(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(url) = Url::parse(trimmed) {
        if matches!(url.scheme(), "http" | "https") {
            return Some(url.to_string());
        }
    }

    let candidate = format!("https://{trimmed}");
    if looks_like_address(trimmed) {
        if let Ok(url) = Url::parse(&candidate) {
            return Some(url.to_string());
        }
    }

    let query = encode_query(trimmed);
    Some(format!("https://duckduckgo.com/?q={query}"))
}

fn looks_like_address(input: &str) -> bool {
    let host_part = input
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .split(':')
        .next()
        .unwrap_or_default();

    host_part.eq_ignore_ascii_case("localhost") || host_part.contains('.')
}

fn encode_query(input: &str) -> String {
    input
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::normalize_navigation_input;

    #[test]
    fn normalizes_empty_domains_urls_and_search_queries() {
        assert_eq!(normalize_navigation_input(""), None);
        assert_eq!(
            normalize_navigation_input(" example.com "),
            Some("https://example.com/".to_string())
        );
        assert_eq!(
            normalize_navigation_input("example.com/docs"),
            Some("https://example.com/docs".to_string())
        );
        assert_eq!(
            normalize_navigation_input("https://example.com"),
            Some("https://example.com/".to_string())
        );
        assert_eq!(
            normalize_navigation_input("http://localhost:3000"),
            Some("http://localhost:3000/".to_string())
        );
        assert_eq!(
            normalize_navigation_input("rust webview browser"),
            Some("https://duckduckgo.com/?q=rust%20webview%20browser".to_string())
        );
    }
}
