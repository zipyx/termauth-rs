use tui::{
    backend::Backend, 
    Frame, 
    layout::Rect, 
    style::{Color, Style, Modifier}, 
    text::{Span, Spans}, 
    widgets::{Block, Borders, Wrap, Paragraph}
};

use crate::App;


pub fn draw_help_welcome<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from(vec![
            Span::styled("mode: ", Style::default().fg(Color::White)),
            Span::from("There are two modes, [insert] mode and [normal] mode."),
        ]),
        Spans::from(vec![
            Span::styled("mode [normal]: ", Style::default().fg(Color::LightBlue)),
            Span::from("Allows for movement across the application"),
        ]),
        Spans::from(vec![
            Span::styled("mode [insert]: ", Style::default().fg(Color::Yellow)),
            Span::from("Allows you to start inserting text into a field"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] j : ", Style::default().fg(Color::LightBlue)),
            Span::from("Scroll text down"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] k : ", Style::default().fg(Color::LightBlue)),
            Span::from("Scroll text up"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] l : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move tab right"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] h : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move tab left"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] q : ", Style::default().fg(Color::LightBlue)),
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


pub fn draw_help_signup<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from(vec![
            Span::styled("[insert] username: ", Style::default().fg(Color::Yellow)),
            Span::from("The username must contain at least 1 lowercase letter, 1 uppercase letter, 1 number, and/or _ ."),
        ]),
        Spans::from(vec![
            Span::styled("[insert] password: ", Style::default().fg(Color::Yellow)),
            Span::from("The password must be at least 8 characters long and must be unique"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] j : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move cursor down, if an option is presented"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] k : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move cursor up, if an option is presented"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] l : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move tab right"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] h : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move tab left"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] q : ", Style::default().fg(Color::LightBlue)),
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

pub fn draw_help_login<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from(vec![
            Span::styled("[insert] username: ", Style::default().fg(Color::Yellow)),
            Span::from("Your username used to sign up"),
        ]),
        Spans::from(vec![
            Span::styled("[insert] password: ", Style::default().fg(Color::Yellow)),
            Span::from("Your password used to sign up"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] j : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move cursor down, if an option is presented"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] k : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move cursor up, if an option is presented"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] l : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move tab right"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] h : ", Style::default().fg(Color::LightBlue)),
            Span::from("Move tab left"),
        ]),
        Spans::from(vec![
            Span::styled("[normal] q : ", Style::default().fg(Color::LightBlue)),
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
