use tui::{
    backend::Backend, 
    Frame, 
    layout::{Rect, Layout, Constraint, Direction, Alignment}, 
    widgets::{
        Paragraph, 
        Block, 
        Borders, 
        ListItem, 
        List, Tabs, Wrap, BorderType
    }, 
    style::{ Style, Modifier, Color }, 
    text::{Span, Spans, Text}};
use chrono::{DateTime, Local, Utc};

use crate::{App, component::{block::centered_rect_a, item::{LOGS, UserMode, Credential}, state}, component::item::MENU};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Yellow))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("System"))
        .highlight_style(Style::default().fg(Color::Green))
        .select(app.tabs.index);

    // Render object
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_welcome(f, app, chunks[1]),
        1 => draw_signup(f, app, chunks[1]),
        2 => draw_login(f, app, chunks[1]),
        3 => draw_notepad(f, app, chunks[1]),
        4 => draw_credential_manager(f, app, chunks[1]),
        _ => {}
    }
}

fn draw_welcome<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ].as_ref(),
        ).split(area);

    draw_welcome_instruction(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);
}

fn draw_welcome_instruction<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let user_choice = 0;
    let size = f.size();
    let main_area = centered_rect_a(40, 14, size);
    let info_area = Rect::new(main_area.left(), main_area.bottom() + 1, main_area.width, 8);

    let menu_items = LOGS
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            if i == user_choice {
                ListItem::new(Span::styled(
                    item,
                    Style::default().add_modifier(Modifier::REVERSED),
                ))
            } else {
                ListItem::new(item)
            }
        }).collect::<Vec<ListItem>>();

    // create menu block with title and color
    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Instructions"))
        .highlight_style(Style::default().fg(Color::Green));

    // render menu in the main area
    f.render_widget(menu, main_area);

    // render information text for menu
    let menu_text = vec![
        Spans::from(vec![
            Span::styled("Sign Up: ", Style::default().fg(Color::Yellow)),
            Span::from("Username & password"),
        ]),
        Spans::from(vec![
            Span::styled("Login  : ", Style::default().fg(Color::Yellow)),
            Span::from("Username & password"),
        ]),
        Spans::from(vec![
            Span::styled("Exit   : ", Style::default().fg(Color::Yellow)),
            Span::from("Quit the application"),
        ]),
        Spans::from(vec![
            Span::styled("q: ", Style::default().fg(Color::Yellow)),
            Span::from("Quit"),
        ]),
    ];

    // build help block containing title and color
    let help_block = Paragraph::new(menu_text)
        .block(Block::default().borders(Borders::ALL).title("Man Page"));

    // Render help block with the help text
    f.render_widget(help_block, info_area);
}

fn draw_signup<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ].as_ref(),
        ).split(area);

    draw_signup_block(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);

}

fn draw_signup_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let a = "";
    let b = "";

    let username_pos = centered_rect_a(40, 8, f.size());
    let password_pos = Rect::new(username_pos.left(), username_pos.bottom(), username_pos.width, 4);

    let username_input = Paragraph::new(a.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Username"))
        .style(match app.user_mode {
            UserMode::Insert => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        });

    f.render_widget(username_input, username_pos);

    let password_input = Paragraph::new(b.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Password"))
        .style(match app.user_mode {
            UserMode::Insert => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        });

    f.render_widget(password_input, password_pos);

}

fn draw_login<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ].as_ref(),
        ).split(area);

    draw_login_block(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);
}

fn draw_login_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref(),
        ).split(area);

    // The below respresents the left side of tab 2
    {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref(),
            ).direction(Direction::Horizontal) .split(chunks[0]);

        let menu_display: Vec<ListItem> = app
            .info.items.iter().map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
            .collect();

        let menu = List::new(menu_display)
            .block(Block::default().borders(Borders::ALL).title("Errors"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(menu, chunks[0], &mut app.info.state);
    }

    // The below respresents the right side of tab 2
    {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref(),
            ).direction(Direction::Horizontal) .split(chunks[0]);

        let menu_display: Vec<ListItem> = app
            .info.items.iter().map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
            .collect();

        let menu = List::new(menu_display)
            .block(Block::default().borders(Borders::ALL).title("Errors"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(menu, chunks[1], &mut app.info.state);
    }

}

fn draw_notepad<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ].as_ref(),
        ).split(area);

    draw_notepad_block(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);

}

fn draw_notepad_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ].as_ref(),
        ).split(area);

    let (msg, style) = match app.user_mode {
        UserMode::Normal => (
            vec![
                Span::raw("You are now in "),
                Span::styled("Normal Mode", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(". Press the "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" key to enter "),
                Span::styled("Insert Mode", Style::default().add_modifier(Modifier::BOLD)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Insert => (
            vec![
                Span::raw("You are now in "),
                Span::styled("Insert Mode", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(". Press the "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" key to enter "),
                Span::styled("Normal Mode", Style::default().add_modifier(Modifier::BOLD)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::SignUp => (
            vec![],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::LogIn => (
            vec![],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Credential => (
            vec![],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
    };

    let mut username = Text::from(Spans::from(msg.to_owned()));
    username.patch_style(style);
    let help_message = Paragraph::new(username);
    f.render_widget(help_message, chunks[0]);

    // Get the user input and display style associated with different modes
    let username_input = Paragraph::new(app.input.as_ref())
        .style(match app.user_mode {
            UserMode::Insert => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        }).block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(username_input, chunks[1]);

    match app.user_mode {
        UserMode::Insert => {
            f.set_cursor
                (
                    chunks[1].x + app.input.len() as u16 + 1,
                    chunks[1].y + 1,
                )
        }
        _ => {}
    }

    let utc: DateTime<Local> = Local::now();

    let credentials: Vec<ListItem> = app
        .notepad
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{} - {}", utc, m)))];
            ListItem::new(content)
        }).collect();

    let credentials = List::new(credentials).block(Block::default().borders(Borders::ALL).title("Notepad"));
    f.render_widget(credentials, chunks[2]);
}

fn draw_credential_manager<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ].as_ref(),
        ).split(area);

    // draw_help_popup(f, app, chunks[2]);
    draw_credential_manager_block(f, app, chunks[1]);
    draw_help(f, app, chunks[2]);

}

fn draw_credential_manager_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let app_pos = centered_rect_a(40, 8, f.size());
    let username_pos = Rect::new(app_pos.left(), app_pos.bottom(), app_pos.width, 4);
    let password_pos = Rect::new(username_pos.left(), username_pos.bottom(), username_pos.width, 4);


    let a = "";
    let b = "";
    let c = "";

    // let app_input = Paragraph::new(app.new_user_app.as_ref())
    let app_input = Paragraph::new(a.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("App"))
        .style(match app.user_mode {
            UserMode::Insert => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        });

    f.render_widget(app_input, app_pos);

    // let username_input = Paragraph::new(app.new_user_username.as_ref())
    let username_input = Paragraph::new(b.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Username"))
        .style(match app.user_mode {
            UserMode::Insert => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        });

    f.render_widget(username_input, username_pos);

    // let password_input = Paragraph::new(app.new_user_password.as_ref())
    let password_input = Paragraph::new(c.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Password"))
        .style(match app.user_mode {
            UserMode::Insert => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        });

    f.render_widget(password_input, password_pos);

}

fn draw_hello_world<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let size = f.size();
    let main_area = centered_rect_a(20, 14, size);

    let paragraph_block = Paragraph::new("Hello, World!")
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Welcome!"));
    f.render_widget(paragraph_block, main_area);
}


fn draw_popup<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let user_choice = 0;
    let size = f.size();
    let main_area = centered_rect_a(20, 14, size);
    let info_area = Rect::new(main_area.left(), main_area.bottom() + 1, main_area.width, 8);
    let menu_items = MENU
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            if i == user_choice {
                ListItem::new(Span::styled(
                    item,
                    Style::default().add_modifier(Modifier::REVERSED),
                ))
            } else {
                ListItem::new(item)
            }
        }).collect::<Vec<ListItem>>();

    // create menu block with title and color
    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(Style::default().fg(Color::Green));

    // render menu in the main area
    f.render_widget(menu, main_area);

    // render information text for menu
    let menu_text = vec![
        Spans::from(vec![
            Span::styled("k: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor up"),
        ]),
        Spans::from(vec![
            Span::styled("j: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor down"),
        ]),
        Spans::from(vec![
            Span::styled("a: ", Style::default().fg(Color::Yellow)),
            Span::from("toggle menu on/off"),
        ]),
        Spans::from(vec![
            Span::styled("q: ", Style::default().fg(Color::Yellow)),
            Span::from("Quit"),
        ]),
    ];

    // build help block containing title and color
    let help_block = Paragraph::new(menu_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));

    // Render help block with the help text
    f.render_widget(help_block, info_area);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from(vec![
            Span::styled("mode: ", Style::default().fg(Color::Yellow)),
            Span::from("There are two modes, insert mode and normal mode."),
        ]),
        Spans::from(vec![
            Span::styled("l: ", Style::default().fg(Color::Yellow)),
            Span::from("Move tab right"),
        ]),
        Spans::from(vec![
            Span::styled("h: ", Style::default().fg(Color::Yellow)),
            Span::from("Move tab left"),
        ]),
        Spans::from(vec![
            Span::styled("q: ", Style::default().fg(Color::Yellow)),
            Span::from("Quit"),
        ]),
    ];

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Help",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    ));

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_help_popup<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let user_choice = 0;
    let size = f.size();
    let main_area = centered_rect_a(40, 14, size);
    let info_area = Rect::new(main_area.left(), main_area.bottom() + 1, main_area.width, 8);

    let menu_items = MENU
        .iter()
        .enumerate()
        .map(|(i, &item)| {
            if i == user_choice {
                ListItem::new(Span::styled(
                    item,
                    Style::default().add_modifier(Modifier::REVERSED),
                ))
            } else {
                ListItem::new(item)
            }
        }).collect::<Vec<ListItem>>();

    // create menu block with title and color
    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(Style::default().fg(Color::Green));

    // render menu in the main area
    f.render_widget(menu, main_area);

    // render information text for menu
    let menu_text = vec![
        Spans::from(vec![
            Span::styled("k: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor up"),
        ]),
        Spans::from(vec![
            Span::styled("j: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor down"),
        ]),
        Spans::from(vec![
            Span::styled("a: ", Style::default().fg(Color::Yellow)),
            Span::from("toggle menu on/off"),
        ]),
        Spans::from(vec![
            Span::styled("q: ", Style::default().fg(Color::Yellow)),
            Span::from("Quit"),
        ]),
    ];

    // build help block containing title and color
    let help_block = Paragraph::new(menu_text)
        .block(Block::default().borders(Borders::ALL).title("Help"));

    // Render help block with the help text
    f.render_widget(help_block, info_area);
}

