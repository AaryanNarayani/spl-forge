use colored::{ColoredString, Colorize};

pub fn app_name(text: &str) -> ColoredString {
    text.truecolor(102, 178, 255).bold()
}

pub fn heading(text: &str) -> ColoredString {
    text.truecolor(125, 211, 252).bold()
}

pub fn label(text: &str) -> ColoredString {
    text.truecolor(148, 223, 255)
}

pub fn command(text: &str) -> ColoredString {
    text.truecolor(94, 234, 212).bold()
}

pub fn action(text: &str) -> ColoredString {
    text.truecolor(255, 201, 107).bold()
}

pub fn success(text: &str) -> ColoredString {
    text.truecolor(110, 231, 183).bold()
}

pub fn warning(text: &str) -> ColoredString {
    text.truecolor(251, 191, 114).bold()
}

pub fn error(text: &str) -> ColoredString {
    text.truecolor(248, 113, 113).bold()
}

pub fn muted(text: &str) -> ColoredString {
    text.truecolor(148, 163, 184)
}

pub fn tip(text: &str) -> ColoredString {
    text.truecolor(196, 181, 253).italic()
}
