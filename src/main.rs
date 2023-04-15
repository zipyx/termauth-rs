mod ui;
mod component;
mod auth;

use std::{error::Error, io};
use component::item::{Account, Credential, UserMode, MENU};
use component::state::{TabsState, StateList};
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen }, 
    execute, 
    event::{EnableMouseCapture, DisableMouseCapture, KeyCode, self, Event}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};


pub struct App<'a> {
    input: String,
    account: Account<'a>,
    credential: Credential<'a>,
    user_mode: UserMode,
    search_query: String,
    search_credentials_list: Vec<Credential<'a>>,
    notepad: Vec<String>,
    tabs: TabsState<'a>,
    info: StateList<&'a str>,
}

impl <'a>App<'a> {
    fn new() -> App<'a> {
        App {
            input: String::new(),
            account: Account::new(),
            credential: Credential::new(),
            user_mode: UserMode::Normal,
            search_query: String::new(),
            search_credentials_list: Vec::new(),
            notepad: Vec::new(),
            tabs: TabsState::new(MENU.to_vec()),
            info: StateList::all_items(MENU.to_vec()),
        }
    }

    fn on_up_account(&mut self) {
        self.account.on_up();
    }

    fn on_down_account(&mut self) {
        self.account.on_down();
    }

    fn on_up_credential(&mut self) {
        self.credential.on_up();
    }

    fn on_down_credential(&mut self) {
        self.credential.on_down();
    }

    fn on_up_info(&mut self) {
        self.info.previous();
    }

    fn on_down_info(&mut self) {
        self.info.next();
    }

    fn on_right(&mut self) {
        self.tabs.next();
    }

    fn on_left(&mut self) {
        self.tabs.previous();
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
    let res = ui_app(&mut terminal, app);
    // let res = ui_popup(&mut terminal, app);
    // let res = ui_menu(&mut terminal, app);
    // let res = ui_selection(&mut terminal, app);
    // let res = ui_signup(&mut terminal, app);

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

    loop {

        terminal.draw(|f| ui::app::draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {

            // Welcome
            if app.tabs.index == 0 {
                match app.user_mode {
                    UserMode::Normal => match key.code {
                        KeyCode::Char('h') => app.on_left(),
                        KeyCode::Char('l') => app.on_right(),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            return Ok(())
                        }
                        _ => {}
                    }
                    _ => {}
                }

            // Sign Up
            } else if app.tabs.index == 1 {
                match app.user_mode {
                    UserMode::Normal => match key.code {
                        KeyCode::Char('j') => app.on_down_account(),
                        KeyCode::Char('k') => app.on_up_account(),
                        KeyCode::Char('h') => app.on_left(),
                        KeyCode::Char('l') => app.on_right(),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            return Ok(())
                        }
                        // KeyCode::Char('i') => {
                        //     app.user_mode = UserMode::Insert;
                        // }
                        _ => {}
                    }
                    _ => {}
                }

            // Login
            } else if app.tabs.index == 2 {
                match app.user_mode {
                    UserMode::Normal => match key.code {
                        KeyCode::Char('j') => app.on_down_info(),
                        KeyCode::Char('k') => app.on_up_info(),
                        KeyCode::Char('h') => app.on_left(),
                        KeyCode::Char('l') => app.on_right(),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            return Ok(())
                        }
                        // KeyCode::Char('i') => {
                        //     app.user_mode = UserMode::Insert;
                        // }
                        _ => {}
                    }
                    _ => {}
                }

            // Notepad
            } else if app.tabs.index == 3 {
                match app.user_mode {
                    UserMode::Normal => match key.code {
                        KeyCode::Char('h') => app.on_left(),
                        KeyCode::Char('l') => app.on_right(),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            return Ok(())
                        }
                        KeyCode::Char('i') => {
                            app.user_mode = UserMode::Insert;
                        }
                        _ => {}
                    }
                    UserMode::Insert => match key.code {
                        KeyCode::Enter => {
                            app.notepad.push(app.input.drain(..).collect());
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.user_mode = UserMode::Normal;
                        }
                        _ => {}
                    }
                    _ => {}
                }

            // Credential Manager
            } else if app.tabs.index == 4 {
                match app.user_mode {
                    UserMode::Normal => match key.code {
                        KeyCode::Char('j') => app.on_down_credential(),
                        KeyCode::Char('k') => app.on_up_credential(),
                        KeyCode::Char('h') => app.on_left(),
                        KeyCode::Char('l') => app.on_right(),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            return Ok(())
                        }
                        // KeyCode::Char('i') => {
                        //     app.user_mode = UserMode::Insert;
                        // }
                        _ => {}
                    }
                    _ => {}
                }
            }
        }

        // if let Event::Key(key) = event::read()? {
        //     match app.user_mode {
        //         UserMode::Normal => match key.code {
        //             KeyCode::Char('j') => app.on_down_info(),
        //             KeyCode::Char('k') => app.on_up_info(),
        //             KeyCode::Char('h') => app.on_left(),
        //             KeyCode::Char('l') => app.on_right(),
        //             KeyCode::Char('q') | KeyCode::Esc => {
        //                 disable_raw_mode()?;
        //                 terminal.show_cursor()?;
        //                 return Ok(())
        //             }
        //             KeyCode::Char('i') => {
        //                 app.user_mode = UserMode::Insert;
        //             }
        //             _ => {}
        //         }
        //         UserMode::Insert => match key.code {
        //             KeyCode::Enter => {
        //                 app.notepad.push(app.input.drain(..).collect());
        //             }
        //             KeyCode::Char(c) => {
        //                 app.input.push(c);
        //             }
        //             KeyCode::Backspace => {
        //                 app.input.pop();
        //             }
        //             KeyCode::Esc => {
        //                 app.user_mode = UserMode::Normal;
        //             }
        //             _ => {}
        //         }
        //         UserMode::SignUp => match key.code {
        //             KeyCode::Char('j') => app.on_down_account(),
        //             KeyCode::Char('k') => app.on_up_account(),
        //             KeyCode::Char('h') => app.on_left(),
        //             KeyCode::Char('l') => app.on_right(),
        //             KeyCode::Char('q') | KeyCode::Esc => {
        //                 disable_raw_mode()?;
        //                 terminal.show_cursor()?;
        //                 return Ok(())
        //             }
        //             _ => {}
        //         }
        //         UserMode::LogIn => match key.code {
        //             KeyCode::Char('j') => app.on_down_account(),
        //             KeyCode::Char('k') => app.on_up_account(),
        //             KeyCode::Char('h') => app.on_left(),
        //             KeyCode::Char('l') => app.on_right(),
        //             KeyCode::Char('q') | KeyCode::Esc => {
        //                 disable_raw_mode()?;
        //                 terminal.show_cursor()?;
        //                 return Ok(())
        //             }
        //             _ => {}
        //         }
        //         UserMode::Credential => match key.code {
        //             KeyCode::Char('j') => app.on_down_credential(),
        //             KeyCode::Char('k') => app.on_up_credential(),
        //             KeyCode::Char('h') => app.on_left(),
        //             KeyCode::Char('l') => app.on_right(),
        //             KeyCode::Char('q') | KeyCode::Esc => {
        //                 disable_raw_mode()?;
        //                 terminal.show_cursor()?;
        //                 return Ok(())
        //             }
        //             _ => {}
        //         }
        //     }
        // }
    }
}


        // if let Event::Key(key) = event::read()? {
        //     // terminal.draw(|f| ui::menu::ui(f, &app, user_choice))?;
        //     match app.menu {
        //         Menu::Main => match key.code {
        //             KeyCode::Char('q') | KeyCode::Esc => {
        //                 disable_raw_mode()?;
        //                 terminal.show_cursor()?;
        //                 return Ok(())
        //             } 

        //             KeyCode::Enter => {
        //                 if user_choice == 0 {
        //                     app.menu = Menu::Signup;
        //                 } else if user_choice == 1 {
        //                     app.menu = Menu::Login;
        //                 } else  {
        //                     disable_raw_mode()?;
        //                     terminal.show_cursor()?;
        //                     return Ok(())
        //                 }
        //             }
        //             KeyCode::Char('j') => {
        //                 if user_choice < 2 {
        //                     user_choice += 1;
        //                 }
        //             }
        //             KeyCode::Char('k') => {
        //                 if user_choice > 0 {
        //                     user_choice -= 1;
        //                 }
        //             }
        //             _ => {}
        //         }
        //         Menu::Login => match key.code {
        //             KeyCode::Esc => {
        //                 app.menu = Menu::Main;
        //             } 
        //             KeyCode::Char('q') => {
        //                 disable_raw_mode()?;
        //                 terminal.show_cursor()?;
        //                 return Ok(())
        //             } 
        //             KeyCode::Char('j') => {
        //                 if user_choice < 1 {
        //                     user_choice += 1;
        //                 }
        //             }
        //             KeyCode::Char('k') => {
        //                 if user_choice > 0 {
        //                     user_choice -= 1;
        //                 }
        //             }
        //             _ => {}
        //         }
        //         Menu::Signup => match key.code {
        //             KeyCode::Esc => {
        //                 app.menu = Menu::Main;
        //             } 
        //             KeyCode::Char('q') => {
        //                 disable_raw_mode()?;
        //                 terminal.show_cursor()?;
        //                 return Ok(())
        //             } 
        //             KeyCode::Char('j') => {
        //                 if user_choice < 1 {
        //                     user_choice += 1;
        //                 }
        //             }
        //             KeyCode::Char('k') => {
        //                 if user_choice > 0 {
        //                     user_choice -= 1;
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // }


// // ui popup run check
// fn _ui_popup<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
//     loop {
//         terminal.draw(|f| ui::popup::ui(f, &app))?;

//         if let Event::Key(key) = event::read()? {
//             match key.code {
//                 KeyCode::Char('q') => return Ok(()),
//                 KeyCode::Char('a') => app.signup = !app.signup,
//                 _ => {}
//             }
//         }
//     }
// }

// // ui menu run check
// fn _ui_menu<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
//     loop {
//         let mut _user_choice = 0;
//         terminal.draw(|f| ui::menu::ui(f, &app, _user_choice))?;

//         if let Event::Key(key) = event::read()? {
//             match key.code {
//                 KeyCode::Char('q') => return Ok(()),
//                 KeyCode::Char('a') => app.signup = !app.signup,
//                 _ => {}
//             }
//         }
//     }
// }

// // run some tests on the terminal for keys pressed
// fn _ui_selection<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

//     let mut user_choice = 0;
//     loop {
//         terminal.draw(|f| ui::menu::ui(f, &app, user_choice))?;

//         if let Event::Key(key) = event::read()? {
//             match key.code {
//                 KeyCode::Char('q') | KeyCode::Esc => {
//                     disable_raw_mode()?;
//                     terminal.show_cursor()?;
//                     return Ok(())
//                 } 
//                 KeyCode::Enter => {
//                     if user_choice == 0 {
//                         disable_raw_mode()?;
//                         app.signup = !app.signup;
//                         // ui_signup(terminal, app);
//                     } else if user_choice == 1 {
//                         terminal.show_cursor()?;
//                         // app.signup = !app.signup;
//                         // ui_signup(terminal, app);
//                     } else  {
//                         disable_raw_mode()?;
//                         terminal.show_cursor()?;
//                         return Ok(())
//                     }
//                 }
//                 KeyCode::Char('a') => app.signup = !app.signup,
//                 KeyCode::Char('j') => {
//                     if user_choice < 2 {
//                         user_choice += 1;
//                     }
//                 }
//                 KeyCode::Char('k') => {
//                     if user_choice > 0 {
//                         user_choice -= 1;
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }         
// }

// // Check if we get the input
// fn _ui_signup<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {

//     // let normal_mode = UserMode::Normal;
//     // let insert_mode = UserMode::Insert;
//     let mut user_choice = 0;
//     let mut input = String::new();
//     let prompt = ">> ";
//     loop {

//         terminal.draw(|f| ui::signup::ui(f, &app, prompt, &input, user_choice))?;

//         if let Event::Key(key) = event::read()? {
//             match app.user_mode {
//                 UserMode::Normal => match key.code {
//                     KeyCode::Char('q') | KeyCode::Esc => {
//                         disable_raw_mode()?;
//                         terminal.show_cursor()?;
//                         return Ok(())
//                     } 
//                     KeyCode::Char('i') => {
//                         app.user_mode = UserMode::Insert;
//                     }
//                     KeyCode::Char('j') => {
//                         if user_choice < 1 {
//                             user_choice += 1;
//                         }
//                     }
//                     KeyCode::Char('k') => {
//                         if user_choice > 0 {
//                             user_choice -= 1;
//                         }
//                     }
//                     _ => {}
//                 }
//                 UserMode::Insert => match key.code {
//                     KeyCode::Esc => {
//                         app.user_mode = UserMode::Normal;
//                     }
//                     KeyCode::Char('q') => {
//                         disable_raw_mode()?;
//                         terminal.show_cursor()?;
//                         return Ok(())
//                     } 
//                     KeyCode::Char(c) => {
//                         input.push(c);
//                     }
//                     KeyCode::Backspace => {
//                         input.pop();
//                     }
//                     KeyCode::Enter => {
//                         terminal.show_cursor()?;
//                         return Ok(())
//                     }
//                     _ => {}
//                 }
//             }

//             KeyCode::Backspace => todo!(),
//             KeyCode::Enter => todo!(),
//             KeyCode::Left => todo!(),
//             KeyCode::Right => todo!(),
//             KeyCode::Up => todo!(),
//             KeyCode::Down => todo!(),
//             KeyCode::Home => todo!(),
//             KeyCode::End => todo!(),
//             KeyCode::PageUp => todo!(),
//             KeyCode::PageDown => todo!(),
//             KeyCode::Tab => todo!(),
//             KeyCode::BackTab => todo!(),
//             KeyCode::Delete => todo!(),
//             KeyCode::Insert => todo!(),
//             KeyCode::F(_) => todo!(),
//             KeyCode::Null => todo!(),
//             KeyCode::Esc => todo!(),
//             KeyCode::CapsLock => todo!(),
//             KeyCode::ScrollLock => todo!(),
//             KeyCode::NumLock => todo!(),
//             KeyCode::PrintScreen => todo!(),
//             KeyCode::Pause => todo!(),
//             KeyCode::Menu => todo!(),
//             KeyCode::KeypadBegin => todo!(),
//             KeyCode::Media(_) => todo!(),
//             KeyCode::Modifier(_) => todo!(),

//         }
//     }
// }






