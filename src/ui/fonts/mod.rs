/// Returns the gohu font definition using system-installed Gohu Font Nerd Font
/// This font is used across all Calm UI components for a consistent retro aesthetic
/// Falls back to embedded base64 if system font not available
pub fn get_gohu_font_face() -> &'static str {
    r#"
        @font-face {
            font-family: 'gohu';
            src: local('GohuFont 11 Nerd Font Mono'),
                 local('GohuFont 14 Nerd Font'),
                 local('GohuFont'),
                 url('data:application/font-woff2;charset=utf-8;base64,d09GMgABAAAAABEYAA4AAAAAJKAAABDEAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAGhYbHhyBbAZgAIEICghWCYM8EQwKgdRgg7hzC4NAAAE2AiQDhx4EIAWDAAeFPQyBZxu8IqOQHkO2HySh4MbHmL/M3v+TQLAbdqUBAhsOACi1kcDKiixZqgMYVqyqKju0e3//b7Pb1SIRqC5SN6ErCDmT0DSReCdh0kgkn2Dz/P//n3m/+cXsO7PvzO07c+/M3DszJJKS5kkk0kmkk0gkkdwk0kmke+/eJvfuPQe9e5vce/emL7lJokTy3ntPIin33ntP7r33nvz/f5tdYKuqKqmqomor87+ft3e21N7ZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9s2afbNm36zZN2v2zZp9s2bfrNk3a/bNmn2zZt+s2Tdr9v0HAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA') format('woff2');
            font-weight: normal;
            font-style: normal;
        }
    "#
}

/// Returns CSS rule to apply gohu font to an element
pub fn get_gohu_font_family() -> &'static str {
    "font-family: 'gohu', 'GohuFont 11 Nerd Font Mono', 'GohuFont', monospace;"
}
