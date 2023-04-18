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
            Span::styled("mode: ", Style::default().fg(Color::Yellow)),
            Span::from("There are two modes, insert mode and normal mode."),
        ]),
        Spans::from(vec![
            Span::styled("j: ", Style::default().fg(Color::Yellow)),
            Span::from("Scroll text down"),
        ]),
        Spans::from(vec![
            Span::styled("k: ", Style::default().fg(Color::Yellow)),
            Span::from("Scroll text up"),
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


pub fn draw_help_signup<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from(vec![
            Span::styled("mode: ", Style::default().fg(Color::Yellow)),
            Span::from("There are two modes, insert mode and normal mode."),
        ]),
        Spans::from(vec![
            Span::styled("username: ", Style::default().fg(Color::Yellow)),
            Span::from("The username must contain at least 1 lowercase letter, 1 uppercase letter, 1 number, and/or _ ."),
        ]),
        Spans::from(vec![
            Span::styled("password: ", Style::default().fg(Color::Yellow)),
            Span::from("The password must be at least 8 characters long and must be unique"),
        ]),
        Spans::from(vec![
            Span::styled("j: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor down, if an option is presented"),
        ]),
        Spans::from(vec![
            Span::styled("k: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor up, if an option is presented"),
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

fn draw_help_login<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from(vec![
            Span::styled("mode: ", Style::default().fg(Color::Yellow)),
            Span::from("There are two modes, insert mode and normal mode."),
        ]),
        Spans::from(vec![
            Span::styled("username: ", Style::default().fg(Color::Yellow)),
            Span::from("The username must contain at least 1 lowercase letter, 1 uppercase letter, 1 number, and/or _ ."),
        ]),
        Spans::from(vec![
            Span::styled("password: ", Style::default().fg(Color::Yellow)),
            Span::from("The password must be at least 8 characters long and must be unique"),
        ]),
        Spans::from(vec![
            Span::styled("j: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor down, if an option is presented"),
        ]),
        Spans::from(vec![
            Span::styled("k: ", Style::default().fg(Color::Yellow)),
            Span::from("Move cursor up, if an option is presented"),
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
