use chrono::{DateTime, Local};
use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout, Direction}, 
    widgets::{Block, Borders, ListItem, List, Paragraph}, 
    style::{Style, Modifier, Color}, text::{Spans, Span, Text}
};

use crate::{App, backend};
use super::utility::helper::draw_help_welcome;
use backend::service::user::UserMode;

pub fn draw_notepad<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(1),
                Constraint::Length(11),
            ].as_ref(),
        ).split(area);

    // let chunks = Layout::default()
    //     .constraints(
    //         [
    //             Constraint::Length(9),
    //             Constraint::Min(8),
    //             Constraint::Length(7),
    //         ].as_ref(),
    //     ).split(area);

    draw_notepad_block(f, app, chunks[1]);
    draw_help_welcome(f, app, chunks[2]);

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
        _ => (vec![], Style::default()),
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

