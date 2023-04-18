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
        user::{User, UserMode, SignUp, Login, CredentialManager},
        utility::constants::{VISITOR, MEMBER, SYSTEM},
    };


// fn main() -> Result<(), Box<dyn Error>> {
pub struct App<'a> {
    user: User,
    scroll: u16,
    tabs_granted: TabsState<'a>,
    tabs_restricted: TabsState<'a>,
    info: StateList<&'a str>,
}

impl <'a>App<'a> {

    fn new() -> App<'a> {

        App {
            user: User::new(),
            scroll: 0,
            tabs_granted: TabsState::new(MEMBER.to_vec()),
            tabs_restricted: TabsState::new(VISITOR.to_vec()),
            info: StateList::all_items(SYSTEM.to_vec()),
        }
    }

    fn tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 50;
    }

    fn on_up_info(&mut self) {
        self.info.previous();
    }

    fn on_down_info(&mut self) {
        self.info.next();
    }

    fn on_right(&mut self) {
        if self.user.signed_in {
            self.tabs_granted.next();
        } else {
            self.tabs_restricted.next();
        }
    }

    fn on_left(&mut self) {
        if self.user.signed_in {
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
    let tick_rate = Duration::from_millis(250);

    // Create app 
    let app = App::new();
    let res = ui_app(&mut terminal, app, tick_rate);

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

fn ui_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, tick_rate: Duration) -> io::Result<()> {

    let mut last_tick = Instant::now();

    loop {

        terminal.draw(|f| ui::app::draw(f, &mut app))?;

        let timeout = tick_rate.checked_sub(
            last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));

        // Handing timeout events
        if crossterm::event::poll(timeout)? {

            // Handle events
            if let Event::Key(key) = event::read()? {

                // if user signed in
                if app.user.signed_in {

                    // Welcome
                    if app.tabs_granted.index == 0 {
                        match app.user.user_mode {
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
                        match app.user.user_mode {
                            UserMode::Normal => match key.code {
                                KeyCode::Char('h') => app.on_left(),
                                KeyCode::Char('l') => app.on_right(),
                                KeyCode::Char('q') | KeyCode::Esc => {
                                    disable_raw_mode()?;
                                    terminal.show_cursor()?;
                                    return Ok(())
                                }
                                KeyCode::Char('i') => {
                                    app.user.user_mode = UserMode::Insert;
                                }
                                _ => {}
                            }
                            UserMode::Insert => match key.code {
                                KeyCode::Enter => {
                                    app.user.notepad.push(app.user.scratchpad.drain(..).collect());
                                }
                                KeyCode::Char(c) => {
                                    app.user.scratchpad.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.user.scratchpad.pop();
                                }
                                KeyCode::Esc => {
                                    app.user.user_mode = UserMode::Normal;
                                }
                                _ => {}
                            }
                            _ => {}
                        }

                        // Credential Manager
                    } else if app.tabs_granted.index == 2 {
                        match app.user.user_mode {

                            UserMode::Normal => match key.code {
                                KeyCode::Char('j') => {
                                    match app.user.app {
                                        CredentialManager::App => {
                                            app.user.app = CredentialManager::Username;
                                        }
                                        CredentialManager::Username => {
                                            app.user.app = CredentialManager::Password;
                                        }
                                        CredentialManager::Password => {
                                            app.user.app = CredentialManager::App;
                                        }
                                    }
                                }
                                KeyCode::Char('k') => {
                                    match app.user.app {
                                        CredentialManager::App => {
                                            app.user.app = CredentialManager::Password;
                                        }
                                        CredentialManager::Username => {
                                            app.user.app = CredentialManager::App;
                                        }
                                        CredentialManager::Password => {
                                            app.user.app = CredentialManager::Username;
                                        }
                                    }
                                }
                                KeyCode::Char('h') => app.on_left(),
                                KeyCode::Char('l') => app.on_right(),
                                KeyCode::Char('q') | KeyCode::Esc => {
                                    disable_raw_mode()?;
                                    terminal.show_cursor()?;
                                    return Ok(())
                                }
                                KeyCode::Char('i') => {
                                    // app.user.user_mode = UserMode::App;
                                    match app.user.app {
                                        CredentialManager::App => {
                                            app.user.user_mode = UserMode::App;
                                            app.user.app = CredentialManager::App;
                                        }
                                        CredentialManager::Username => {
                                            app.user.user_mode = UserMode::Username;
                                            app.user.app = CredentialManager::Username;
                                        }
                                        CredentialManager::Password => {
                                            app.user.user_mode = UserMode::Password;
                                            app.user.app = CredentialManager::Password;
                                        }
                                    }
                                }
                                _ => {}
                            }

                            UserMode::App => match key.code {
                                KeyCode::Enter => {
                                    app.user.set_app_name(app.user.app_name.to_owned());
                                    app.user.user_mode = UserMode::Username;
                                    app.user.app = CredentialManager::Username;
                                }
                                KeyCode::Char(c) => {
                                    app.user.app_name.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.user.app_name.pop();
                                }
                                KeyCode::Esc => {
                                    app.user.user_mode = UserMode::Normal;
                                }
                                _ => {}
                            }

                            UserMode::Username => match key.code {
                                KeyCode::Enter => {
                                    app.user.set_app_username(app.user.app_username.to_owned());
                                    app.user.user_mode = UserMode::Password;
                                    app.user.app = CredentialManager::Password;
                                }
                                KeyCode::Char(c) => {
                                    app.user.app_username.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.user.app_username.pop();
                                }
                                KeyCode::Esc => {
                                    app.user.user_mode = UserMode::Normal;
                                }
                                _ => {}
                            }

                            UserMode::Password => match key.code {
                                KeyCode::Enter => {
                                    app.user.set_app_password(app.user.app_secure_password.to_owned());
                                    app.user.user_mode = UserMode::Normal;
                                }
                                KeyCode::Char(c) => {
                                    // let ast: char = '*';
                                    app.user.app_password.push(c);
                                    app.user.app_secure_password.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.user.app_password.pop();
                                    app.user.app_secure_password.pop();
                                }
                                KeyCode::Esc => {
                                    app.user.user_mode = UserMode::Normal;
                                }
                                _ => {}
                            }
                            _ => {}

                        }


                    } // end of tabs granted
                }  // end of signed in

                // Welcome
                if app.tabs_restricted.index == 0 {
                    match app.user.user_mode {
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

                    match app.user.user_mode {

                        UserMode::Normal => match key.code {
                            KeyCode::Char('w') => {
                                app.user.create_account(
                                    app.user.signup_username.to_owned(), 
                                    app.user.signup_secure_password.to_owned()
                                );
                                app.user.signup_username.clear();
                                app.user.signup_password.clear();
                                app.user.signup_secure_password.clear();
                                app.user.signed_in = true;
                            }
                            KeyCode::Char('j') => {
                                app.user.signup = SignUp::Password;
                            }
                            KeyCode::Char('k') => {
                                app.user.signup = SignUp::Username;
                            }
                            KeyCode::Char('h') => app.on_left(),
                            KeyCode::Char('l') => app.on_right(),
                            KeyCode::Char('q') | KeyCode::Esc => {
                                disable_raw_mode()?;
                                terminal.show_cursor()?;
                                return Ok(())
                            }
                            KeyCode::Char('i') => {
                                match app.user.signup {
                                    SignUp::Username => {
                                        app.user.user_mode = UserMode::Username;
                                        app.user.signup = SignUp::Username;
                                    }
                                    SignUp::Password => {
                                        app.user.user_mode = UserMode::Password;
                                        app.user.signup = SignUp::Password;
                                    }
                                }
                            }
                            KeyCode::Char('p') => {
                                app.user.user_mode = UserMode::Password;
                                app.user.signup = SignUp::Password;
                            }
                            _ => {}
                        }

                        // Signup Username
                        UserMode::Username => match key.code {
                            KeyCode::Enter => {
                                app.user.user_mode = UserMode::Password;
                                app.user.signup = SignUp::Password;
                            }
                            KeyCode::Char(c) => {
                                app.user.signup_username.push(c);
                            }
                            KeyCode::Backspace => {
                                app.user.signup_username.pop();
                            }
                            KeyCode::Esc => {
                                app.user.user_mode = UserMode::Normal;
                            }
                            _ => {}
                        }

                        // Signup Password
                        UserMode::Password => match key.code {
                            KeyCode::Enter => {
                                app.user.user_mode = UserMode::Normal;
                            }
                            KeyCode::Char(c) => {
                                let ast: char = '*';
                                app.user.signup_password.push(ast);
                                app.user.signup_secure_password.push(c);
                            }
                            KeyCode::Backspace => {
                                app.user.signup_password.pop();
                                app.user.signup_secure_password.pop();
                            }
                            KeyCode::Esc => {
                                app.user.user_mode = UserMode::Normal;
                            }
                            _ => {}
                        }
                        _ => {}
                    }

                    // Login
                } else if app.tabs_restricted.index == 2 {
                    match app.user.user_mode {

                        UserMode::Normal => match key.code {
                            KeyCode::Char('w') => {
                                app.user.login(
                                    app.user.login_username.to_owned(), 
                                    app.user.login_secure_password.to_owned()
                                );
                                app.user.login_username.clear();
                                app.user.login_password.clear();
                                app.user.login_secure_password.clear();
                                app.user.signed_in = true;
                            }
                            KeyCode::Char('j') => {
                                app.user.login = Login::Password;
                            }
                            KeyCode::Char('k') => {
                                app.user.login = Login::Username;
                            }
                            KeyCode::Char('h') => app.on_left(),
                            KeyCode::Char('l') => app.on_right(),
                            KeyCode::Char('q') | KeyCode::Esc => {
                                disable_raw_mode()?;
                                terminal.show_cursor()?;
                                return Ok(())
                            }
                            KeyCode::Char('i') => {
                                match app.user.login {
                                    Login::Username => {
                                        app.user.user_mode = UserMode::Username;
                                        app.user.login = Login::Username;
                                    }
                                    Login::Password => {
                                        app.user.user_mode = UserMode::Password;
                                        app.user.login = Login::Password;
                                    }
                                }
                            }
                            KeyCode::Char('p') => {
                                app.user.user_mode = UserMode::Password;
                                app.user.login = Login::Password;
                            }
                            _ => {}
                        }

                        // Login Username
                        UserMode::Username => match key.code {
                            KeyCode::Enter => {
                                app.user.set_login_username(app.user.login_username.to_owned());
                                app.user.user_mode = UserMode::Password;
                                app.user.login = Login::Password;
                            }
                            KeyCode::Char(c) => {
                                app.user.login_username.push(c);
                            }
                            KeyCode::Backspace => {
                                app.user.login_username.pop();
                            }
                            KeyCode::Esc => {
                                app.user.user_mode = UserMode::Normal;
                            }
                            _ => {}
                        }

                        // Login Password
                        UserMode::Password => match key.code {
                            KeyCode::Enter => {
                                app.user.set_login_password(app.user.login_secure_password.to_owned());
                                app.user.user_mode = UserMode::Normal;
                            }
                            KeyCode::Char(c) => {
                                let ast: char = '*';
                                app.user.login_password.push(ast);
                                app.user.login_secure_password.push(c);
                            }
                            KeyCode::Backspace => {
                                app.user.login_password.pop();
                                app.user.login_secure_password.pop();
                            }
                            KeyCode::Esc => {
                                app.user.user_mode = UserMode::Normal;
                            }
                            _ => {}
                        } // User mode parenthesis
                        _ => {}
                    } // match area

                } // end of else if

            } // end of handle events = main loop
        }


        if last_tick + tick_rate <= Instant::now() {
            app.tick();
            last_tick = Instant::now();
        }
    }
}

