use alloc::string::String;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeComponent {
    Icons,
    Fonts,
    Colors,
}

pub struct SigmaThemeManager2;
