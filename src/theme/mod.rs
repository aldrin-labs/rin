use std::fmt;

// Colors
pub struct Colors {
    // Background colors
    pub bg_primary: &'static str,
    pub bg_secondary: &'static str,
    pub bg_input: &'static str,
    pub bg_button: &'static str,
    pub bg_button_active: &'static str,

    // Text colors
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_success: &'static str,
    pub text_danger: &'static str,
    pub text_muted: &'static str,

    // Border colors
    pub border: &'static str,

    // Brand colors
    pub brand_primary: &'static str,
    pub brand_secondary: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ThemeVariant {
    Dark,
    SolarizedYellow,
    HighContrast,
    RetroCrt,
    Matrix,
}

impl fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThemeVariant::Dark => write!(f, "Dark"),
            ThemeVariant::SolarizedYellow => write!(f, "Solarized Yellow"),
            ThemeVariant::HighContrast => write!(f, "High Contrast"),
            ThemeVariant::RetroCrt => write!(f, "Retro CRT"),
            ThemeVariant::Matrix => write!(f, "Matrix"),
        }
    }
}

impl ThemeVariant {
    pub fn colors(&self) -> &'static Colors {
        match self {
            ThemeVariant::Dark => &DARK_THEME,
            ThemeVariant::SolarizedYellow => &SOLARIZED_YELLOW_THEME,
            ThemeVariant::HighContrast => &HIGH_CONTRAST_THEME,
            ThemeVariant::RetroCrt => &RETRO_CRT_THEME,
            ThemeVariant::Matrix => &MATRIX_THEME,
        }
    }

    pub fn all() -> &'static [(ThemeVariant, &'static str)] {
        &[
            (ThemeVariant::Dark, "Dark"),
            (ThemeVariant::SolarizedYellow, "Solarized Yellow"),
            (ThemeVariant::HighContrast, "High Contrast"),
            (ThemeVariant::RetroCrt, "Retro CRT"),
            (ThemeVariant::Matrix, "Matrix"),
        ]
    }
}

// Available themes
pub const DARK_THEME: Colors = Colors {
    bg_primary: "#000022",
    bg_secondary: "#000044",
    bg_input: "#000044",
    bg_button: "#D0D0D0",
    bg_button_active: "#0088FF",
    text_primary: "#FFFFFF",
    text_secondary: "#000022",
    text_success: "#00FF00",
    text_danger: "#FF0000",
    text_muted: "#808080",
    border: "#1A1A3A",
    brand_primary: "#00FF00",
    brand_secondary: "#0088FF",
};

pub const SOLARIZED_YELLOW_THEME: Colors = Colors {
    bg_primary: "#002B36",    // Base03
    bg_secondary: "#073642",  // Base02
    bg_input: "#073642",      // Base02
    bg_button: "#B58900",     // Yellow
    bg_button_active: "#CB4B16", // Orange
    text_primary: "#FDF6E3",  // Base3
    text_secondary: "#002B36", // Base03
    text_success: "#859900",  // Green
    text_danger: "#DC322F",   // Red
    text_muted: "#839496",    // Base0
    border: "#586E75",        // Base01
    brand_primary: "#B58900", // Yellow
    brand_secondary: "#268BD2", // Blue
};

pub const HIGH_CONTRAST_THEME: Colors = Colors {
    bg_primary: "#000000",
    bg_secondary: "#0A0A0A",
    bg_input: "#0A0A0A",
    bg_button: "#FFFFFF",
    bg_button_active: "#FFFF00",
    text_primary: "#FFFFFF",
    text_secondary: "#000000",
    text_success: "#00FF00",
    text_danger: "#FF0000",
    text_muted: "#CCCCCC",
    border: "#FFFFFF",
    brand_primary: "#FFFF00",
    brand_secondary: "#00FFFF",
};

pub const RETRO_CRT_THEME: Colors = Colors {
    bg_primary: "#0000AA",    // Classic Borland Blue
    bg_secondary: "#000088",
    bg_input: "#000088",
    bg_button: "#55FF55",     // Phosphor Green
    bg_button_active: "#FFFF55", // Yellow
    text_primary: "#55FF55",  // Phosphor Green
    text_secondary: "#000088",
    text_success: "#55FF55",  // Phosphor Green
    text_danger: "#FF5555",   // Light Red
    text_muted: "#55FFFF",    // Cyan
    border: "#55FF55",        // Phosphor Green
    brand_primary: "#FFFF55", // Yellow
    brand_secondary: "#55FFFF", // Cyan
};

pub const MATRIX_THEME: Colors = Colors {
    bg_primary: "#000000",
    bg_secondary: "#001100",
    bg_input: "#001100",
    bg_button: "#00FF00",
    bg_button_active: "#00AA00",
    text_primary: "#00FF00",
    text_secondary: "#000000",
    text_success: "#00FF00",
    text_danger: "#FF0000",
    text_muted: "#007700",
    border: "#003300",
    brand_primary: "#00FF00",
    brand_secondary: "#00AA00",
};

pub struct Theme;

impl Theme {
    // Base colors
    pub const BG_PRIMARY: &'static str = "bg-[var(--bg-primary)]";
    pub const BG_SECONDARY: &'static str = "bg-[var(--bg-secondary)]";
    pub const BG_INPUT: &'static str = "bg-[var(--bg-input)]";
    pub const BG_BUTTON: &'static str = "bg-[var(--button)]";
    pub const BG_BUTTON_ACTIVE: &'static str = "bg-[var(--button-active)]";
    pub const TEXT_PRIMARY: &'static str = "text-[var(--text-primary)]";
    pub const TEXT_SECONDARY: &'static str = "text-[var(--text-secondary)]";
    pub const TEXT_SUCCESS: &'static str = "text-[var(--text-success)]";
    pub const TEXT_DANGER: &'static str = "text-[var(--text-danger)]";
    pub const TEXT_MUTED: &'static str = "text-[var(--text-muted)]";

    // Typography
    pub const TEXT_XS: &'static str = "text-xs";
    pub const TEXT_SM: &'static str = "text-sm";
    pub const TEXT_BASE: &'static str = "text-base";
    pub const FONT_MONO: &'static str = "font-mono";
    pub const FONT_MEDIUM: &'static str = "font-medium";
    pub const FONT_BOLD: &'static str = "font-bold";

    // Layout
    pub const FLEX: &'static str = "flex";
    pub const FLEX_COL: &'static str = "flex flex-col";
    pub const ITEMS_CENTER: &'static str = "items-center";
    pub const JUSTIFY_BETWEEN: &'static str = "justify-between";

    // Borders
    pub const BORDER: &'static str = "border border-[var(--border)]";
    pub const BORDER_T: &'static str = "border-t border-[var(--border)]";
    pub const BORDER_B: &'static str = "border-b border-[var(--border)]";
    pub const ROUNDED: &'static str = "rounded";
    pub const ROUNDED_T: &'static str = "rounded-t";
    pub const ROUNDED_L: &'static str = "rounded-l";
    pub const ROUNDED_R: &'static str = "rounded-r";

    // Helper functions
    pub fn cx(classes: &[&str]) -> String {
        classes.join(" ")
    }

    pub fn button(variant: ButtonVariant, extra: &str) -> String {
        match variant {
            ButtonVariant::Primary => Self::cx(&[
                "px-4 py-2",
                Self::ROUNDED,
                Self::BG_BUTTON,
                Self::TEXT_SECONDARY,
                extra
            ]),
            ButtonVariant::Secondary => Self::cx(&[
                "px-4 py-2",
                Self::ROUNDED,
                Self::BG_BUTTON_ACTIVE,
                Self::TEXT_PRIMARY,
                extra
            ]),
        }
    }

    pub fn input(extra: &str) -> String {
        Self::cx(&[
            "w-full p-1.5",
            Self::ROUNDED,
            Self::BG_INPUT,
            Self::TEXT_PRIMARY,
            Self::FONT_MONO,
            extra
        ])
    }

    pub fn card(extra: &str) -> String {
        Self::cx(&[
            Self::BG_PRIMARY,
            Self::BORDER,
            Self::ROUNDED,
            extra
        ])
    }

    pub fn select(extra: &str) -> String {
        Self::cx(&[
            "px-3 py-1.5",
            Self::ROUNDED,
            Self::BG_BUTTON,
            Self::TEXT_SECONDARY,
            "outline-none cursor-pointer",
            "hover:bg-[var(--button-active)] hover:text-[var(--text-primary)]",
            extra
        ])
    }
}

pub enum ButtonVariant {
    Primary,
    Secondary,
}

// CSS Variables
pub fn get_css_variables(colors: &Colors) -> String {
    format!(":root {{
            --bg-primary: {};
            --bg-secondary: {};
            --bg-input: {};
            --button: {};
            --button-active: {};
            --text-primary: {};
            --text-secondary: {};
            --text-success: {};
            --text-danger: {};
            --text-muted: {};
            --border: {};
            --brand-primary: {};
            --brand-secondary: {};
        }}

        * {{
            transition: background-color 0.2s, color 0.2s, border-color 0.2s;
        }}

        select {{
            background-image: url(\"data:image/svg+xml;charset=utf-8,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3E%3Cpath stroke='%23{}' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3E%3C/svg%3E\");
            background-position: right 0.2rem center;
            background-repeat: no-repeat;
            background-size: 24px 24px;
        }}

        select option {{
            background-color: {};
            color: {};
            padding: 0.5rem;
        }}

        select:focus {{
            outline: none;
            border-color: {};
        }}",
        colors.bg_primary,
        colors.bg_secondary,
        colors.bg_input,
        colors.bg_button,
        colors.bg_button_active,
        colors.text_primary,
        colors.text_secondary,
        colors.text_success,
        colors.text_danger,
        colors.text_muted,
        colors.border,
        colors.brand_primary,
        colors.brand_secondary,
        &colors.text_muted[1..],
        colors.bg_secondary,
        colors.text_primary,
        colors.brand_primary,
    )
}
