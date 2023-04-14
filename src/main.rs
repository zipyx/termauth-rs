mod ui;
mod component;
mod auth;

use std::{error::Error, io};
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen }, 
    execute, 
    event::{EnableMouseCapture, DisableMouseCapture, KeyCode, self, Event}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};


pub struct App<'a> {
    _input: String,
    input_mode: component::item::InputMode,
    menu: component::item::Menu,
    signup: bool,
    test: component::state::StateList<&'a str>,
    tabs: component::state::TabsState<'a>,
    servers: Vec<component::state::Server<'a>>,
}

impl <'a>App<'a> {
    fn new() -> App<'a> {
        App {
            _input: String::new(),
            input_mode: component::item::InputMode::Normal,
            menu: component::item::Menu::Main,
            signup: false,
            test: component::state::StateList::all_items(component::item::MENU.to_vec()),
            tabs: component::state::TabsState::new(vec!["Sign Up", "Login", "Quit"]),
            servers: vec![
                component::state::Server {
                    name: "Server 1",
                    location: "ap-southeast-1",
                    coords: (0.0, 0.0),
                    status: "Online",
                },
                component::state::Server {
                    name: "Server 2",
                    location: "ap-southeast-2",
                    coords: (0.0, 0.0),
                    status: "Offline",
                },
                component::state::Server {
                    name: "ap-southeast-3",
                    location: "US",
                    coords: (0.0, 0.0),
                    status: "Online",
                },
            ]
        }
    }

    fn on_up(&mut self) {
        self.test.previous();
    }

    fn on_down(&mut self) {
        self.test.next();
    }

    fn on_right(&mut self) {
        self.tabs.next();
    }

    fn on_left(&mut self) {
        self.tabs.previous();
    }
}

// impl App {
//     fn new() -> App {
//         App {
//             _input: String::new(),
//             input_mode: InputMode::Normal,
//             tabs: TabsState::new(vec!["Sign Up", "Login", "Quit"]),
//             menu: Menu::Main,
//             signup: false,
//         }
//     }
// }


fn main() -> Result<(), Box<dyn Error>> {

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app 
    let app = App::new();
    // let res = ui_popup(&mut terminal, app);
    // let res = ui_menu(&mut terminal, app);
    // let res = ui_selection(&mut terminal, app);
    // let res = ui_signup(&mut terminal, app);
    let res = ui_app(&mut terminal, app);

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

fn ui_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

    let mut user_choice = 0;
    let mut input = String::new();
    let prompt = ">> ";
    loop {

        // terminal.draw(|f| ui::signup::ui(f, &app, prompt, &input, user_choice))?;
        terminal.draw(|f| ui::menu::ui(f, &app, user_choice))?;

        if let Event::Key(key) = event::read()? {
            // terminal.draw(|f| ui::menu::ui(f, &app, user_choice))?;
            match app.menu {
                component::item::Menu::Main => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        return Ok(())
                    } 

                    KeyCode::Enter => {
                        if user_choice == 0 {
                            app.menu = component::item::Menu::Signup;
                        } else if user_choice == 1 {
                            app.menu = component::item::Menu::Login;
                        } else  {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            return Ok(())
                        }
                    }
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
                component::item::Menu::Login => match key.code {
                    KeyCode::Esc => {
                        app.menu = component::item::Menu::Main;
                    } 
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        return Ok(())
                    } 
                    KeyCode::Char('j') => {
                        if user_choice < 1 {
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
                component::item::Menu::Signup => match key.code {
                    KeyCode::Esc => {
                        app.menu = component::item::Menu::Main;
                    } 
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        return Ok(())
                    } 
                    KeyCode::Char('j') => {
                        if user_choice < 1 {
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
}

// ui popup run check
fn _ui_popup<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
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
fn _ui_menu<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let mut _user_choice = 0;
        terminal.draw(|f| ui::menu::ui(f, &app, _user_choice))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('a') => app.signup = !app.signup,
                _ => {}
            }
        }
    }
}

// run some tests on the terminal for keys pressed
fn ui_selection<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

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
                        disable_raw_mode()?;
                        app.signup = !app.signup;
                        // ui_signup(terminal, app);
                    } else if user_choice == 1 {
                        terminal.show_cursor()?;
                        // app.signup = !app.signup;
                        // ui_signup(terminal, app);
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

// Check if we get the input
fn ui_signup<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

    // let normal_mode = component::item::InputMode::Normal;
    // let insert_mode = component::item::InputMode::Insert;
    let mut user_choice = 0;
    let mut input = String::new();
    let prompt = ">> ";
    loop {

        terminal.draw(|f| ui::signup::ui(f, &app, prompt, &input, user_choice))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                component::item::InputMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        return Ok(())
                    } 
                    KeyCode::Char('i') => {
                        app.input_mode = component::item::InputMode::Insert;
                    }
                    KeyCode::Char('j') => {
                        if user_choice < 1 {
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
                component::item::InputMode::Insert => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = component::item::InputMode::Normal;
                    }
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        return Ok(())
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
                    _ => {}
                }
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






