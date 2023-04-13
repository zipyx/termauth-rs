mod ui;
mod components;
mod auth;

use std::{error::Error, io, sync::mpsc};
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen }, 
    execute, 
    event::{EnableMouseCapture, DisableMouseCapture, KeyCode, self, Event}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};


enum InputMode {
    Normal,
    Insert,
}

pub struct App {
    // current value of the input box
    input: String,
    // current input mode
    input_mode: InputMode,
    signup: bool,
    _login: bool,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            signup: false,
            _login: false,
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app 
    let app = App::new();
    // let res = run_popup(&mut terminal, app);
    // let res = run_menu(&mut terminal, app);
    // let res = run_test(&mut terminal, app);
    let res = next_test(&mut terminal, app);

    // Restore terminal on exit
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    // Handle errors
    if let Err(e) = res {
        println!("Error: {}", e);
    }

    // Return result
    Ok(())
}

// ui popup run check
fn _run_popup<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::popup::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('a') => app.signup = !app.signup,
                _ => {}
            }
        }
    }
}

// ui menu run check
fn _run_menu<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        // terminal.draw(|f| ui::menu::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('a') => app.signup = !app.signup,
                _ => {}
            }
        }
    }
}

// run some tests on the terminal on keys pressed
fn run_test<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

    let mut user_choice = 0;
    loop {
        terminal.draw(|f| ui::menu::ui(f, &app, user_choice))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    return Ok(())
                } 
                KeyCode::Enter => {
                    if user_choice == 0 {
                        terminal.show_cursor()?;
                        app.signup = !app.signup;
                    } else if user_choice == 1 {
                        terminal.show_cursor()?;
                        app.signup = !app.signup;
                    } else  {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        return Ok(())
                    }
                }
                KeyCode::Char('a') => app.signup = !app.signup,
                KeyCode::Char('j') => {
                    if user_choice < 2 {
                        user_choice += 1;
                    }
                }
                KeyCode::Char('k') => {
                    if user_choice > 0 {
                        user_choice -= 1;
                    }
                }
                _ => {}
            }
        }
    }         
}

enum _EventKeys<I> {
    Input(I),
}

// Check if we get the input
fn next_test<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {

    let mut user_choice = 0;
    let mut input = String::new();
    let prompt = ">> ";
    loop {
        terminal.draw(|f| auth::signup::ui(f, &app, prompt, &input, user_choice))?;

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('J') => {
                    if user_choice < 2 {
                        user_choice += 1;
                    }
                }
                KeyCode::Char('K') => {
                    if user_choice > 0 {
                        user_choice -= 1;
                    }
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Enter => {
                    terminal.show_cursor()?;
                    return Ok(())
                }
                KeyCode::Esc => {
                    terminal.show_cursor()?;
                    return Ok(())
                }
                _ => {}
            }
            // KeyCode::Backspace => todo!(),
            // KeyCode::Enter => todo!(),
            // KeyCode::Left => todo!(),
            // KeyCode::Right => todo!(),
            // KeyCode::Up => todo!(),
            // KeyCode::Down => todo!(),
            // KeyCode::Home => todo!(),
            // KeyCode::End => todo!(),
            // KeyCode::PageUp => todo!(),
            // KeyCode::PageDown => todo!(),
            // KeyCode::Tab => todo!(),
            // KeyCode::BackTab => todo!(),
            // KeyCode::Delete => todo!(),
            // KeyCode::Insert => todo!(),
            // KeyCode::F(_) => todo!(),
            // KeyCode::Null => todo!(),
            // KeyCode::Esc => todo!(),
            // KeyCode::CapsLock => todo!(),
            // KeyCode::ScrollLock => todo!(),
            // KeyCode::NumLock => todo!(),
            // KeyCode::PrintScreen => todo!(),
            // KeyCode::Pause => todo!(),
            // KeyCode::Menu => todo!(),
            // KeyCode::KeypadBegin => todo!(),
            // KeyCode::Media(_) => todo!(),
            // KeyCode::Modifier(_) => todo!(),
        }
    }
}






