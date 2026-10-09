const SHELL_HTML: &str = include_str!("../assets/shell.html");
const STYLES: &str = include_str!("../assets/styles.css");
const SCRIPT: &str = include_str!("../assets/shell.js");

pub fn browser_shell_html(home_url: &str) -> String {
    SHELL_HTML
        .replace("{{HOME_URL}}", home_url)
        .replace("{{STYLES}}", STYLES)
        .replace("{{SCRIPT}}", SCRIPT)
}

#[cfg(test)]
mod tests {
    use super::browser_shell_html;

    #[test]
    fn embeds_browser_shell_controls_and_assets() {
        let html = browser_shell_html("https://duckduckgo.com");

        assert!(html.contains("id=\"address\""));
        assert!(html.contains("data-home=\"https://duckduckgo.com\""));
        assert!(html.contains("shell.js"));
        assert!(html.contains("styles.css"));
    }
}
