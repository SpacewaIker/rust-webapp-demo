use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

// theme source color on material theme builder: #00a9b2
impl Theme {
    pub fn get_theme(&self) -> &ThemeStyle {
        static DARK: Lazy<ThemeStyle> = Lazy::new(|| ThemeStyle {
            primary: String::from("#4cd9e3"),
            on_primary: String::from("#00363a"),
            primary_container: String::from("#004f54"),
            on_primary_container: String::from("#72f5ff"),
            background: String::from("#191c1c"),
            on_background: String::from("#e0e3e3"),
            surface: String::from("#191c1c"),
            on_surface: String::from("#e0e3e3"),
            outline: String::from("#899393"),
            error: String::from("#ffb4ab"),
            on_error: String::from("#690005"),
            secondary: String::from("#b1cbce"),
            on_secondary: String::from("#1b3436"),
            tertiary: String::from("#b6c7ea"),
            on_tertiary: String::from("#20314c"),
        });

        static LIGHT: Lazy<ThemeStyle> = Lazy::new(|| ThemeStyle {
            primary: String::from("#00696f"),
            on_primary: String::from("#ffffff"),
            primary_container: String::from("#72f5ff"),
            on_primary_container: String::from("#002022"),
            background: String::from("#fafdfc"),
            on_background: String::from("#191c1c"),
            surface: String::from("#fafdfc"),
            on_surface: String::from("#191c1c"),
            outline: String::from("#6f797a"),
            error: String::from("#ba1a1a"),
            on_error: String::from("#ffffff"),
            secondary: String::from("#4a6365"),
            on_secondary: String::from("#ffffff"),
            tertiary: String::from("#4f5f7d"),
            on_tertiary: String::from("#ffffff"),
        });

        match self {
            Theme::Dark => &DARK,
            Theme::Light => &LIGHT,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeStyle {
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,
    pub background: String,
    pub on_background: String,
    pub surface: String,
    pub on_surface: String,
    pub outline: String,
    pub error: String,
    pub on_error: String,
    pub secondary: String,
    pub on_secondary: String,
    pub tertiary: String,
    pub on_tertiary: String,
}
