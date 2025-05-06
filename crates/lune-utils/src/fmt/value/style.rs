use console::Style;

pub static COLOR_GREEN: std::sync::LazyLock<Style> =
    std::sync::LazyLock::new(|| Style::new().green());
pub static COLOR_YELLOW: std::sync::LazyLock<Style> =
    std::sync::LazyLock::new(|| Style::new().yellow());
pub static COLOR_MAGENTA: std::sync::LazyLock<Style> =
    std::sync::LazyLock::new(|| Style::new().magenta());
pub static COLOR_CYAN: std::sync::LazyLock<Style> =
    std::sync::LazyLock::new(|| Style::new().cyan());

pub static STYLE_DIM: std::sync::LazyLock<Style> = std::sync::LazyLock::new(|| Style::new().dim());
