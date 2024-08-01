use derive_setters::*;
use ratatui::{prelude::*, widgets::*};

#[derive(Default, Setters, Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameOption {
    title: String,
    selected: bool,
}

impl Widget for GameOption {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let symbol = if self.selected { ">" } else { " " };

        Paragraph::new(format!("{} {}", symbol, self.title)).render(area, buf)
    }
}
