mod html;
mod script;
mod styles;

pub fn get_download_overlay_html() -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>{}</style>
</head>
<body>
    {}
    <script>{}</script>
</body>
</html>"#,
        styles::get_styles(),
        html::get_html(),
        script::get_script()
    )
}
