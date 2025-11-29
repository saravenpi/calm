mod html;
mod script;
mod styles;

pub use html::get_command_prompt_html_structure;
pub use script::get_command_prompt_script;
pub use styles::get_command_prompt_styles;

pub fn get_command_prompt_html() -> String {
    let styles = get_command_prompt_styles();
    let script = get_command_prompt_script();

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Command Prompt</title>
    <style>
        {}
    </style>
</head>
<body>
    {}
    <script>
        {}
    </script>
</body>
</html>"#,
        styles,
        get_command_prompt_html_structure(),
        script
    )
}
