pub enum AnsiColor {
    _Black,
    _Red,
    _Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    _White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    _BrightMagenta,
    _BrightCyan,
    _BrightWhite,
}

impl AnsiColor {
    fn to_code(&self) -> &str {
        match self {
            AnsiColor::_Black => "\x1b[30m",
            AnsiColor::_Red => "\x1b[31m",
            AnsiColor::_Green => "\x1b[32m",
            AnsiColor::Yellow => "\x1b[33m",
            AnsiColor::Blue => "\x1b[34m",
            AnsiColor::Magenta => "\x1b[35m",
            AnsiColor::Cyan => "\x1b[36m",
            AnsiColor::_White => "\x1b[37m",
            AnsiColor::BrightBlack => "\x1b[90m",
            AnsiColor::BrightRed => "\x1b[91m",
            AnsiColor::BrightGreen => "\x1b[92m",
            AnsiColor::BrightYellow => "\x1b[93m",
            AnsiColor::BrightBlue => "\x1b[94m",
            AnsiColor::_BrightMagenta => "\x1b[95m",
            AnsiColor::_BrightCyan => "\x1b[96m",
            AnsiColor::_BrightWhite => "\x1b[97m",
        }
    }
}

pub fn colorize(text: &str, color: AnsiColor) -> String {
    format!("{}{}\x1b[0m", color.to_code(), text)
}
