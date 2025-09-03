use serde::{Deserialize, Serialize};
use crate::ui::components::modals::multi_select::MultiSelectItem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DateFormat {
    AmPm12Hour,
    AmPm12HourWithYear,
    Hour24,
    Hour24WithYear,
    Iso8601,
    Relative,
}

impl Default for DateFormat {
    fn default() -> Self {
        DateFormat::AmPm12Hour
    }
}

impl DateFormat {
    pub fn name(&self) -> &'static str {
        match self {
            DateFormat::AmPm12Hour => "12-hour (Dec 15 2:33 pm)",
            DateFormat::AmPm12HourWithYear => "12-hour + Year (Dec 15 2024 2:33 pm)",
            DateFormat::Hour24 => "24-hour (Dec 15 14:33)",
            DateFormat::Hour24WithYear => "24-hour + Year (Dec 15 2024 14:33)",
            DateFormat::Iso8601 => "ISO 8601 (2024-12-15 14:33)",
            DateFormat::Relative => "Relative (2 hours ago)",
        }
    }

    pub fn all() -> Vec<DateFormat> {
        vec![
            DateFormat::AmPm12Hour,
            DateFormat::AmPm12HourWithYear,
            DateFormat::Hour24,
            DateFormat::Hour24WithYear,
            DateFormat::Iso8601,
            DateFormat::Relative,
        ]
    }
}

impl MultiSelectItem for DateFormat {
    fn name(&self) -> &str {
        self.name()
    }

    fn all() -> Vec<Self> {
        Self::all()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorTheme {
    CatppuccinMocha,
    TokyoNight,
    OneDark,
    GruvboxDark,
    Nord,
    Monokai,
    SolarizedDark,
    Dracula,
}

impl Default for ColorTheme {
    fn default() -> Self {
        ColorTheme::CatppuccinMocha
    }
}

impl ColorTheme {
    pub fn name(&self) -> &'static str {
        match self {
            ColorTheme::CatppuccinMocha => "Catppuccin Mocha",
            ColorTheme::TokyoNight => "Tokyo Night",
            ColorTheme::OneDark => "One Dark",
            ColorTheme::GruvboxDark => "Gruvbox Dark",
            ColorTheme::Nord => "Nord",
            ColorTheme::Monokai => "Monokai",
            ColorTheme::SolarizedDark => "Solarized Dark",
            ColorTheme::Dracula => "Dracula",
        }
    }

    pub fn all() -> Vec<ColorTheme> {
        vec![
            ColorTheme::CatppuccinMocha,
            ColorTheme::TokyoNight,
            ColorTheme::OneDark,
            ColorTheme::GruvboxDark,
            ColorTheme::Nord,
            ColorTheme::Monokai,
            ColorTheme::SolarizedDark,
            ColorTheme::Dracula,
        ]
    }

    pub fn next(&self) -> ColorTheme {
        match self {
            ColorTheme::CatppuccinMocha => ColorTheme::TokyoNight,
            ColorTheme::TokyoNight => ColorTheme::OneDark,
            ColorTheme::OneDark => ColorTheme::GruvboxDark,
            ColorTheme::GruvboxDark => ColorTheme::Nord,
            ColorTheme::Nord => ColorTheme::Monokai,
            ColorTheme::Monokai => ColorTheme::SolarizedDark,
            ColorTheme::SolarizedDark => ColorTheme::Dracula,
            ColorTheme::Dracula => ColorTheme::CatppuccinMocha,
        }
    }
}

impl MultiSelectItem for ColorTheme {
    fn name(&self) -> &str {
        self.name()
    }

    fn all() -> Vec<Self> {
        Self::all()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    pub show_completed_count: bool,
    pub auto_expand_descriptions: bool,
    pub compact_mode: bool,
    pub status_bar_visible: bool,
    #[serde(default = "default_split_ratio")]
    pub split_ratio: u16,
    #[serde(default = "default_vim_mode")]
    pub vim_mode: bool,
    #[serde(default)]
    pub date_format: DateFormat,
}

fn default_vim_mode() -> bool {
    false
}

fn default_split_ratio() -> u16 {
    50
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            show_completed_count: true,
            auto_expand_descriptions: false,
            compact_mode: false,
            status_bar_visible: true,
            split_ratio: 50,
            vim_mode: false,
            date_format: DateFormat::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: ColorTheme,
    pub ui: UISettings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ColorTheme::default(),
            ui: UISettings::default(),
        }
    }
}