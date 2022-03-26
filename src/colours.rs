use tui:: style::{Color, Modifier, Style};
use tui::text::{Text, Span};
use lazy_static::lazy_static;
use super::bobbles::Bobble;

const PIECE: &str = " ";

lazy_static! {
    pub static ref RED_STYLE: Style = Style::default()
        .fg(Color::Rgb(231, 24, 55))
        .add_modifier(Modifier::BOLD);
    pub static ref GREEN_STYLE: Style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    pub static ref BLUE_STYLE: Style = Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::BOLD);
    pub static ref PURPLE_STYLE: Style = Style::default()
        .fg(Color::Rgb(115, 105, 147))
        .add_modifier(Modifier::BOLD);
    pub static ref PINK_STYLE: Style = Style::default()
        .fg(Color::Rgb(255, 170, 170))
        .add_modifier(Modifier::BOLD);
    pub static ref YELLOW_STYLE: Style = Style::default()
        .fg(Color::Rgb(255, 247, 0))
        .add_modifier(Modifier::BOLD);
    pub static ref WHITE_STYLE: Style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    pub static ref ORANGE_STYLE: Style = Style::default()
        .fg(Color::Rgb(255, 123, 0))
        .add_modifier(Modifier::BOLD);
    pub static ref HEADER_STYLE: Style = Style::default()
        .fg(Color::LightRed)
        .add_modifier(Modifier::BOLD);
}

pub fn get_bobbles_string() -> Text<'static> {
    let mut text = Text::styled(PIECE, *RED_STYLE);
    text.extend(Text::styled(PIECE, *GREEN_STYLE));
    text.extend(Text::styled(PIECE, *BLUE_STYLE));
    text.extend(Text::styled(PIECE, *PURPLE_STYLE));
    text.extend(Text::styled(PIECE, *PINK_STYLE));
    text.extend(Text::styled(PIECE, *YELLOW_STYLE));
    text.extend(Text::styled(PIECE, *WHITE_STYLE));
    text.extend(Text::styled(PIECE, *ORANGE_STYLE));
    text
}

pub fn make_bobbles_span<'a>(bobbles: &Vec<Bobble>) -> Vec<Span<'a>> {
    let mut spans: Vec<Span<'a>> = Vec::new(); 
    use Bobble::*;
    for b in bobbles {
        match b {
            Red => spans.push(Span::styled(PIECE, *RED_STYLE)),
            Green => spans.push(Span::styled(PIECE, *GREEN_STYLE)),
            Blue => spans.push(Span::styled(PIECE, *BLUE_STYLE)),
            Purple => spans.push(Span::styled(PIECE, *PURPLE_STYLE)),
            Pink => spans.push(Span::styled(PIECE, *PINK_STYLE)),
            Yellow => spans.push(Span::styled(PIECE, *YELLOW_STYLE)),
            White => spans.push(Span::styled(PIECE, *WHITE_STYLE)),
            Orange => spans.push(Span::styled(PIECE, *ORANGE_STYLE)),
        }
    } 
    spans
}
