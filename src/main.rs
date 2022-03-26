use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// use std::{io, thread, time::Duration};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tui_mm::app::{App, Outcome};



fn main() -> Result<(), io::Error> {
    // setup terminal
    let mut app = App::new();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let running = true;
    while running {
        terminal.draw(|f| {

            // let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Max(6), Constraint::Min(15), Constraint::Max(3)].as_ref())
                .split(f.size());
            let my_text: Text = app.header.clone();
            let block = Paragraph::new(my_text)
                .block(Block::default().title("Guide").borders(Borders::ALL));
            // let bobbles_string = get_bobbles_string();
            //
            // TODO: Construct list / paragraph here
            // by concatenating (?) app.guesses
            let mut my_text: Text = Text::raw("");
            my_text.extend(app.guesses.clone());
            let block2 = Paragraph::new(my_text).block(Block::default());
            let input_vec = app.input();
            let input_txt = input_vec.iter().collect::<String>();
            let block3 = Paragraph::new(input_txt).block(Block::default().title("Enter your guess").borders(Borders::ALL));
            f.render_widget(block, chunks[0]);
            f.render_widget(block2, chunks[1]);
            f.render_widget(block3, chunks[2]);
            let offset = app.input.len();
            f.set_cursor(chunks[2].x + 1 + offset as u16, chunks[2].y + 1);
        })?;

        let key = read().unwrap();
        match key {
            Event::Key(KeyEvent { code, modifiers: _ }) => match code {
                KeyCode::Esc => break,
                KeyCode::Backspace => app.input_remove_previous(),
                KeyCode::Left => app.left(),
                KeyCode::Right => app.right(),
                KeyCode::Enter => {
                    app.submit_guess();
                    if app.outcome == Outcome::Win { break }
                },
                KeyCode::Char(char) => app.input_write(char),
                _ => {}, 
            }
            _ => {}
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;
    println!("Entered chars: {}", &app.input.drain(..).collect::<String>());
    println!("Input cursor position: {}", &app.input_cursor);

    Ok(())
}

