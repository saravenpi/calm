pub mod html;
pub mod styles;
pub mod script;

pub use html::get_tab_bar_html_structure;
pub use styles::get_tab_bar_styles;
pub use script::get_tab_bar_script;

pub fn get_complete_tab_bar_html_with_opacity(opacity: f32) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Calm Browser</title>
    <style>
        {}
        body {{
            opacity: {};
        }}
    </style>
</head>
<body>
    {}
    <script>
        {}
    </script>
</body>
</html>"#,
        get_tab_bar_styles(),
        opacity,
        get_tab_bar_html_structure(),
        get_tab_bar_script()
    )
}
