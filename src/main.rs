use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// use std::{io, thread, time::Duration};
use std::io;
use tui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Rect, Constraint, Direction, Layout},
    text::{Text, Span, Spans},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame, Terminal,
};
use tui_mm::app::{App, GameState};

const MAX_TURNS: u8 = 12;

fn main() -> Result<(), io::Error> {
    // setup terminal
    let mut app = App::new();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // start main loop
    let running = true;
    while running {
        terminal.draw(|f| ui(f, &mut app))?;

        let key = read().unwrap();
        match key {
            Event::Key(KeyEvent { code, modifiers: _ }) => match (code, &app.game_state) {
                (KeyCode::Esc, _) => break,
                (KeyCode::Backspace, GameState::InProgress) => app.input_remove_previous(),
                (KeyCode::Left, GameState::InProgress) => app.left(),
                (KeyCode::Right, GameState::InProgress) => app.right(),
                (KeyCode::Enter, GameState::InProgress) => app.submit_guess(),
                (KeyCode::Char(char), GameState::InProgress) => app.input_write(char),
                (_, _) => {}
            },
            _ => {}
        }
        if app.turn == MAX_TURNS {
            app.game_state = GameState::Loss;
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    // println!(
    //     "Entered chars: {}",
    //     &app.input.drain(..).collect::<String>()
    // );
    // println!("Input cursor position: {}", &app.input_cursor);

    Ok(())
}

fn ui<T: Backend>(f: &mut Frame<T>, app: &mut App) {
    // let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(6), Constraint::Min(15), Constraint::Max(3)].as_ref())
        .split(f.size());

    let my_text: Text = app.header.clone();
    let header =
        Paragraph::new(my_text).block(Block::default().title("Guide").borders(Borders::ALL));
    f.render_widget(header, chunks[0]);
    let mut my_text: Text = Text::raw("");

    my_text.extend(app.guesses.clone());
    let board = Paragraph::new(my_text).block(Block::default());
    f.render_widget(board, chunks[1]);

    let input_vec = app.input();
    let input_txt = input_vec.iter().collect::<String>();
    let input_area = Paragraph::new(input_txt).block(
        Block::default()
            .title("Enter your guess")
            .borders(Borders::ALL),
    );
    f.render_widget(input_area, chunks[2]);

    if app.game_state == GameState::InProgress {
        let offset = app.input.len();
        f.set_cursor(chunks[2].x + 1 + offset as u16, chunks[2].y + 1);
    }
    if app.game_state == GameState::Win {
        let area = centered_rect(26, 5, chunks[1]);
        f.render_widget(Clear, area);
        // A Text is a Text<'a?> { lines: Vec<Spans<'a>> }
        let mut winner_message: Text = Text::styled("    Congratulations!", *tui_mm::colours::RED_STYLE);
        winner_message.extend(Text::styled("  You are a MASTERMIND!", *tui_mm::colours::ORANGE_STYLE));
        let mut bobble_span_start = vec![Span::raw(" The code was ")];
        let bobble_span = tui_mm::colours::make_bobbles_span(&app.secret);
        bobble_span_start.extend(bobble_span);
        let bobble_spans = Spans::from(bobble_span_start);
        let message_end = Text::from(bobble_spans);
        winner_message.extend(message_end);
        let message = Paragraph::new(winner_message).block(
            Block::default()
            .title("You win!")
                .borders(Borders::ALL),);
        f.render_widget(message, area);
    }
    if app.game_state == GameState::Loss {
        let area = centered_rect(29, 5, chunks[1]);
        f.render_widget(Clear, area);
        // A Text is a Text<'a?> { lines: Vec<Spans<'a>> }
        let mut winner_message: Text = Text::styled("  Better luck next time!", *tui_mm::colours::RED_STYLE);
        winner_message.extend(Text::styled(" You are not a MASTERMIND.", *tui_mm::colours::ORANGE_STYLE));
        let mut bobble_span_start = vec![Span::raw("  The code was ")];
        let bobble_span = tui_mm::colours::make_bobbles_span(&app.secret);
        bobble_span_start.extend(bobble_span);
        let bobble_spans = Spans::from(bobble_span_start);
        let message_end = Text::from(bobble_spans);
        winner_message.extend(message_end);
        let message = Paragraph::new(winner_message).block(
            Block::default()
            .title("You lost!")
                .borders(Borders::ALL),);
        f.render_widget(message, area);
    }
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let Rect {
        width: grid_width,
        height: grid_height,
        ..
    } = r;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(grid_height / 2 - height / 2),
                Constraint::Length(height),
                Constraint::Length(grid_height / 2 - height / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(grid_width / 2 - width / 2),
                Constraint::Length(width),
                Constraint::Length(grid_width / 2 - width / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
