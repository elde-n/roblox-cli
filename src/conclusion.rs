use console::Color;

pub(crate) struct Conclusion(pub(crate) bool);

impl std::fmt::Display for Conclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Conclusion {
    pub(crate) fn color(&self) -> Color {
        if self.0 { Color::Green } else { Color::Red }
    }

    pub(crate) fn value(&self) -> &'static str {
        if self.0 { "Yes" } else { "No" }
    }
}
