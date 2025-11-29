pub mod html;
pub mod script;
pub mod styles;

pub use html::get_tab_bar_html_structure;
pub use script::get_tab_bar_script;
pub use styles::get_tab_bar_styles;

pub fn get_complete_tab_bar_html(vim_mode: bool, sounds_enabled: bool) -> String {
    let styles = get_tab_bar_styles();
    let sounds_script = crate::ui::get_sounds_script(sounds_enabled);
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Calm Browser</title>
    <style>
        {}
    </style>
</head>
<body>
    {}
    <script>
        window.vimMode = {};
        {}
        {}
    </script>
</body>
</html>"#,
        styles,
        get_tab_bar_html_structure(),
        vim_mode,
        sounds_script,
        get_tab_bar_script()
    )
}
