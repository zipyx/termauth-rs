use tui::{
    backend::Backend, 
    Frame, 
    layout::{Layout, Constraint}, 
    widgets::{
        Block, 
        Borders, 
        Tabs, 
    }, 
    style::{ Style, Color }, 
    text::{Span, Spans}};

use crate::App;
use super::tabs::{welcome, signup, login, notepad, credential_manager};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3), 
                Constraint::Min(0)
            ].as_ref()
        ) .split(f.size());

    let titles = app
        .user
        .tab
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::White))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("System"))
        .highlight_style(Style::default().fg(Color::Green))
        .select(app.user.tab.index);

    // Render tabs with respective UI
    f.render_widget(tabs, chunks[0]);

    if app.user.get_signed_in() {
        match app.user.tab.index {
            0 => welcome::draw_welcome(f, app, chunks[1]),
            1 => notepad::draw_notepad(f, app, chunks[1]),
            2 => credential_manager::draw_credential_manager(f, app, chunks[1]),
            _ => {}
        }
    } else {
        match app.user.tab.index {
            0 => welcome::draw_welcome(f, app, chunks[1]),
            1 => signup::draw_signup(f, app, chunks[1]),
            2 => login::draw_login(f, app, chunks[1]),
            _ => {}
        }
    }
}
