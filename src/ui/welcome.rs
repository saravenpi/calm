/// Returns the HTML content for the welcome page shown on first launch.
pub fn get_welcome_html() -> String {
    include_str!("welcome.html").to_string()
}
