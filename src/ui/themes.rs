use crate::models::ColorTheme;
use ratatui::style::{Color, Style};

pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub muted: Color,
    pub border: Color,
    pub modal_bg: Color,
    pub modal_border: Color,
    pub vim_normal_bg: Color,
    pub vim_insert_bg: Color,
    pub vim_visual_bg: Color,
    pub vim_command_bg: Color,
    pub vim_text: Color,
}

impl ThemeColors {
    pub fn from_theme(theme: &ColorTheme) -> Self {
        match theme {
            ColorTheme::CatppuccinMocha => Self::catppuccin_mocha(),
            ColorTheme::TokyoNight => Self::tokyo_night(),
            ColorTheme::OneDark => Self::one_dark(),
            ColorTheme::GruvboxDark => Self::gruvbox_dark(),
            ColorTheme::Nord => Self::nord(),
        }
    }

    fn catppuccin_mocha() -> Self {
        Self {
            background: Color::Rgb(24, 24, 37),
            foreground: Color::Rgb(205, 214, 244),
            primary: Color::Rgb(137, 180, 250),
            secondary: Color::Rgb(49, 50, 68),
            accent: Color::Rgb(203, 166, 247),
            success: Color::Rgb(166, 227, 161),
            warning: Color::Rgb(249, 226, 175),
            error: Color::Rgb(243, 139, 168),
            muted: Color::Rgb(127, 132, 156),
            border: Color::Rgb(88, 91, 112),
            modal_bg: Color::Rgb(17, 17, 27),
            modal_border: Color::Rgb(137, 180, 250),
            vim_normal_bg: Color::Rgb(137, 180, 250),    // Blue
            vim_insert_bg: Color::Rgb(166, 227, 161),    // Green  
            vim_visual_bg: Color::Rgb(203, 166, 247),    // Purple/Mauve
            vim_command_bg: Color::Rgb(249, 226, 175),   // Yellow
            vim_text: Color::Rgb(17, 17, 27),            // Dark text
        }
    }

    fn tokyo_night() -> Self {
        Self {
            background: Color::Rgb(26, 27, 38),
            foreground: Color::Rgb(192, 202, 245),
            primary: Color::Rgb(125, 207, 255),
            secondary: Color::Rgb(41, 46, 66),
            accent: Color::Rgb(187, 154, 247),
            success: Color::Rgb(158, 206, 106),
            warning: Color::Rgb(224, 175, 104),
            error: Color::Rgb(247, 118, 142),
            muted: Color::Rgb(86, 95, 137),
            border: Color::Rgb(52, 59, 88),
            modal_bg: Color::Rgb(16, 18, 25),
            modal_border: Color::Rgb(125, 207, 255),
            vim_normal_bg: Color::Rgb(125, 207, 255),    // Cyan
            vim_insert_bg: Color::Rgb(158, 206, 106),    // Green
            vim_visual_bg: Color::Rgb(187, 154, 247),    // Purple
            vim_command_bg: Color::Rgb(224, 175, 104),   // Orange
            vim_text: Color::Rgb(16, 18, 25),            // Dark text
        }
    }

    fn one_dark() -> Self {
        Self {
            background: Color::Rgb(40, 44, 52),
            foreground: Color::Rgb(171, 178, 191),
            primary: Color::Rgb(97, 175, 239),
            secondary: Color::Rgb(55, 59, 65),
            accent: Color::Rgb(198, 120, 221),
            success: Color::Rgb(152, 195, 121),
            warning: Color::Rgb(229, 192, 123),
            error: Color::Rgb(224, 108, 117),
            muted: Color::Rgb(92, 99, 112),
            border: Color::Rgb(76, 82, 99),
            modal_bg: Color::Rgb(32, 35, 42),
            modal_border: Color::Rgb(97, 175, 239),
            vim_normal_bg: Color::Rgb(97, 175, 239),     // Blue
            vim_insert_bg: Color::Rgb(152, 195, 121),    // Green
            vim_visual_bg: Color::Rgb(198, 120, 221),    // Purple
            vim_command_bg: Color::Rgb(229, 192, 123),   // Yellow
            vim_text: Color::Rgb(32, 35, 42),            // Dark text
        }
    }

    fn gruvbox_dark() -> Self {
        Self {
            background: Color::Rgb(40, 40, 40),
            foreground: Color::Rgb(235, 219, 178),
            primary: Color::Rgb(131, 165, 152),
            secondary: Color::Rgb(60, 56, 54),
            accent: Color::Rgb(211, 134, 155),
            success: Color::Rgb(184, 187, 38),
            warning: Color::Rgb(250, 189, 47),
            error: Color::Rgb(251, 73, 52),
            muted: Color::Rgb(146, 131, 116),
            border: Color::Rgb(102, 92, 84),
            modal_bg: Color::Rgb(29, 32, 33),
            modal_border: Color::Rgb(131, 165, 152),
            vim_normal_bg: Color::Rgb(131, 165, 152),    // Teal
            vim_insert_bg: Color::Rgb(184, 187, 38),     // Yellow-green
            vim_visual_bg: Color::Rgb(211, 134, 155),    // Pink
            vim_command_bg: Color::Rgb(250, 189, 47),    // Orange
            vim_text: Color::Rgb(29, 32, 33),            // Dark text
        }
    }

    fn nord() -> Self {
        Self {
            background: Color::Rgb(46, 52, 64),
            foreground: Color::Rgb(236, 239, 244),
            primary: Color::Rgb(136, 192, 208),
            secondary: Color::Rgb(59, 66, 82),
            accent: Color::Rgb(180, 142, 173),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            error: Color::Rgb(191, 97, 106),
            muted: Color::Rgb(76, 86, 106),
            border: Color::Rgb(67, 76, 94),
            modal_bg: Color::Rgb(36, 40, 51),
            modal_border: Color::Rgb(136, 192, 208),
            vim_normal_bg: Color::Rgb(136, 192, 208),    // Frost blue
            vim_insert_bg: Color::Rgb(163, 190, 140),    // Aurora green
            vim_visual_bg: Color::Rgb(180, 142, 173),    // Aurora purple
            vim_command_bg: Color::Rgb(235, 203, 139),   // Aurora yellow
            vim_text: Color::Rgb(36, 40, 51),            // Dark text
        }
    }
}

pub struct ThemeStyles {
    pub normal: Style,
    pub selected: Style,
    pub completed: Style,
    pub title: Style,
    pub description: Style,
    pub border: Style,
    pub status_bar: Style,
    pub help_text: Style,
    pub accent: Style,
    pub muted: Style,
}

impl ThemeStyles {
    pub fn from_colors(colors: &ThemeColors) -> Self {
        Self {
            normal: Style::default().fg(colors.foreground).bg(colors.background),
            selected: Style::default().fg(colors.background).bg(colors.primary),
            completed: Style::default().fg(colors.muted),
            title: Style::default().fg(colors.primary),
            description: Style::default().fg(colors.muted),
            border: Style::default().fg(colors.border),
            status_bar: Style::default().fg(colors.foreground).bg(colors.border),
            help_text: Style::default().fg(colors.muted),
            accent: Style::default().fg(colors.accent),
            muted: Style::default().fg(colors.muted),
        }
    }
}