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
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tui_mm::app::{App, GameState};

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
            Event::Key(KeyEvent { code, modifiers: _ }) => match code {
                KeyCode::Esc => break,
                KeyCode::Backspace => app.input_remove_previous(),
                KeyCode::Left => app.left(),
                KeyCode::Right => app.right(),
                KeyCode::Enter => {
                    app.submit_guess();
                    if app.game_state == GameState::Win {
                        break;
                    }
                }
                KeyCode::Char(char) => app.input_write(char),
                _ => {}
            },
            _ => {}
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    println!(
        "Entered chars: {}",
        &app.input.drain(..).collect::<String>()
    );
    println!("Input cursor position: {}", &app.input_cursor);

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
    let input_vec = app.input();
    let input_txt = input_vec.iter().collect::<String>();
    let input_area = Paragraph::new(input_txt).block(
        Block::default()
            .title("Enter your guess")
            .borders(Borders::ALL),
    );
    f.render_widget(board, chunks[1]);
    f.render_widget(input_area, chunks[2]);

    if app.game_state == GameState::InProgress {
        let offset = app.input.len();
        f.set_cursor(chunks[2].x + 1 + offset as u16, chunks[2].y + 1);
    }
    if app.game_state == GameState::Win {}
    if app.game_state == GameState::Loss {}
}
