mod ui;
mod component;
mod backend;

use std::{error::Error, io, time::{Duration, Instant}};
use component::state::{TabsState, StateList};
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen }, 
    execute, 
    event::{EnableMouseCapture, DisableMouseCapture, KeyCode, self, Event}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};
use backend::
    service::{
        credential,
        credential::Manager,
        user::{UserMode, SignUp, Login, User, UserService},
        utility::constants::{VISITOR, MEMBER, SYSTEM},
    };

struct Temp {
    username: String,
    password: String,
    credential_name: String,
    credential_username: String,
    credential_password: String,
    search_query: String,
    notepad: Vec<String>,
}

// fn main() -> Result<(), Box<dyn Error>> {
pub struct App<'a> {
    input: String,
    user: User,
    user_mode: UserMode,
    // scroll: u16,
    signup: SignUp,
    login: Login,
    signed_in: bool,
    new_username: String,
    new_password: String,
    new_credential_name: String,
    new_credential_username: String,
    new_credential_password: String,
    search_query: String,
    // search_credentials_list: Vec<CredentialManager>,
    notepad: Vec<String>,
    tabs_granted: TabsState<'a>,
    tabs_restricted: TabsState<'a>,
    info: StateList<&'a str>,
}

impl <'a>App<'a> {

    fn new() -> App<'a> {

        App {
            input: String::new(),
            user: User::new(),
            user_mode: UserMode::Normal,
            // scroll: 0,
            signup: SignUp::Username,
            login: Login::Username,
            signed_in: false,
            new_username: String::new(),
            new_password: String::new(),
            new_credential_name: String::new(),
            new_credential_username: String::new(),
            new_credential_password: String::new(),
            search_query: String::new(),
            // search_credentials_list: Vec::new(),
            notepad: Vec::new(),
            tabs_granted: TabsState::new(MEMBER.to_vec()),
            tabs_restricted: TabsState::new(VISITOR.to_vec()),
            info: StateList::all_items(SYSTEM.to_vec()),
        }
    }

    // fn tick(&mut self) {
    //     self.scroll += 1;
    //     self.scroll %= 3;
    // }

    fn on_up_info(&mut self) {
        self.info.previous();
    }

    fn on_down_info(&mut self) {
        self.info.next();
    }

    fn on_right(&mut self) {
        if self.signed_in {
            self.tabs_granted.next();
        } else {
            self.tabs_restricted.next();
        }
    }

    fn on_left(&mut self) {
        if self.signed_in {
            self.tabs_granted.previous();
        } else {
            self.tabs_restricted.previous();
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

    // set timer
    // let tick_rate = Duration::from_millis(250);

    // Create app 
    let app = App::new();
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

    // let mut last_tick = Instant::now();

    loop {

        terminal.draw(|f| ui::app::draw(f, &mut app))?;

        // let timeout = tick_rate.checked_sub(
        //     last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));

        // Handle events
        if let Event::Key(key) = event::read()? {

            // if user signed in
            if app.signed_in {

                // Welcome
                if app.tabs_granted.index == 0 {
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

                    // Notepad
                } else if app.tabs_granted.index == 1 {
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
                } else if app.tabs_granted.index == 2 {
                    match app.user_mode {

                        // UserMode::Normal => match key.code {
                        //     KeyCode::Char('j') => {
                        //         app.login = Login::Password
                        //     }
                        //     KeyCode::Char('k') => {
                        //         app.login = Login::Username;
                        //     }
                        //     KeyCode::Char('h') => app.on_left(),
                        //     KeyCode::Char('l') => app.on_right(),
                        //     KeyCode::Char('q') | KeyCode::Esc => {
                        //         disable_raw_mode()?;
                        //         terminal.show_cursor()?;
                        //         return Ok(())
                        //     }
                        //     _ => {}
                        // }
                        // _ => {}

                        UserMode::Normal => match key.code {
                            KeyCode::Char('j') => {
                                app.signup = SignUp::Password;
                            }
                            KeyCode::Char('k') => {
                                app.signup = SignUp::Username;
                            }
                            KeyCode::Char('h') => app.on_left(),
                            KeyCode::Char('l') => app.on_right(),
                            KeyCode::Char('q') | KeyCode::Esc => {
                                disable_raw_mode()?;
                                terminal.show_cursor()?;
                                return Ok(())
                            }
                            KeyCode::Char('i') => {
                                app.user_mode = UserMode::Username;
                            }
                            KeyCode::Char('p') => {
                                app.user_mode = UserMode::Password;
                                app.signup = SignUp::Password;
                            }
                            _ => {}
                        }

                        UserMode::Username => match key.code {
                            KeyCode::Enter => {
                                app.user.set_username(app.new_username.to_owned());
                                app.user_mode = UserMode::Password;
                                app.signup = SignUp::Password;
                            }
                            KeyCode::Char(c) => {
                                app.new_username.push(c);
                            }
                            KeyCode::Backspace => {
                                app.new_username.pop();
                            }
                            KeyCode::Esc => {
                                app.user_mode = UserMode::Normal;
                            }
                            _ => {}
                        }

                        UserMode::Password => match key.code {
                            KeyCode::Enter => {
                                app.user.set_password(app.new_username.to_owned());
                                app.user_mode = UserMode::Normal;
                            }
                            KeyCode::Char(c) => {
                                app.new_password.push(c);
                            }
                            KeyCode::Backspace => {
                                app.new_password.pop();
                            }
                            KeyCode::Esc => {
                                app.user_mode = UserMode::Normal;
                            }
                            _ => {}
                        }
                        _ => {}

                    }


                } // end of tabs granted
            }  // end of signed in

            // Welcome
            if app.tabs_restricted.index == 0 {
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
            } else if app.tabs_restricted.index == 1 {
                match app.user_mode {

                    UserMode::Normal => match key.code {
                        // KeyCode::Char('j') => app.signup = SignUp::Password,
                        // KeyCode::Char('k') => app.signup = SignUp::Username,
                        KeyCode::Char('w') => {
                            app.user.new_account(
                                app.new_username.to_owned(), 
                                app.new_password.to_owned()
                            );
                            app.new_username.clear();
                            app.new_password.clear();
                            app.user.set_username(app.new_username.to_owned());
                            app.user.set_password(app.new_password.to_owned());
                            app.signed_in = true;
                        }
                        KeyCode::Char('j') => {
                            app.signup = SignUp::Password;
                        }
                        KeyCode::Char('k') => {
                            app.signup = SignUp::Username;
                        }
                        KeyCode::Char('h') => app.on_left(),
                        KeyCode::Char('l') => app.on_right(),
                        KeyCode::Char('q') | KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            return Ok(())
                        }
                        KeyCode::Char('i') => {
                            app.user_mode = UserMode::Username;
                        }
                        KeyCode::Char('p') => {
                            app.user_mode = UserMode::Password;
                            app.signup = SignUp::Password;
                        }
                        _ => {}
                    }
                    
                    UserMode::Username => match key.code {
                        KeyCode::Enter => {
                            app.user.set_username(app.new_username.to_owned());
                            app.user_mode = UserMode::Password;
                            app.signup = SignUp::Password;
                        }
                        KeyCode::Char(c) => {
                            app.new_username.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_username.pop();
                        }
                        KeyCode::Esc => {
                            app.user_mode = UserMode::Normal;
                        }
                        _ => {}
                    }

                    UserMode::Password => match key.code {
                        KeyCode::Enter => {
                            app.user.set_password(app.new_username.to_owned());
                            app.user_mode = UserMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.new_password.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_password.pop();
                        }
                        KeyCode::Esc => {
                            app.user_mode = UserMode::Normal;
                        }
                        _ => {}
                    }
                    _ => {}
                }

                // Login
            } else if app.tabs_restricted.index == 2 {
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
                        _ => {}
                    }
                    _ => {}
                }
            } 

        } // end of handle events = main loop
    }
}

