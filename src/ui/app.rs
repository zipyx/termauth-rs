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
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    // Render object
    // f.render_widget(tabs, chunks[0]);
    if app.user.signed_in{

        // All granted tabs with their respective titles
        // temp stuff : TODO link this to actual feature component
        let granted_titles = app
            .tabs_granted
            .titles
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Yellow))))
            .collect();
        let tabs_granted = Tabs::new(granted_titles)
            .block(Block::default().borders(Borders::ALL).title("System"))
            .highlight_style(Style::default().fg(Color::Green))
            .select(app.tabs_granted.index);

        f.render_widget(tabs_granted, chunks[0]);
        match app.tabs_granted.index {
            0 => welcome::draw_welcome(f, app, chunks[1]),
            1 => notepad::draw_notepad(f, app, chunks[1]),
            2 => credential_manager::draw_credential_manager(f, app, chunks[1]),
            _ => {}
        }

    } else {

        // Restrict access to certain tabs if not signed in
        // temp stuff : TODO link this to actual feature component
        let restricted_titles = app
            .tabs_restricted
            .titles
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Yellow))))
            .collect();
        let tabs_restricted = Tabs::new(restricted_titles)
            .block(Block::default().borders(Borders::ALL).title("System"))
            .highlight_style(Style::default().fg(Color::Green))
            .select(app.tabs_restricted.index);

        f.render_widget(tabs_restricted, chunks[0]);
        match app.tabs_restricted.index {
            0 => welcome::draw_welcome(f, app, chunks[1]),
            1 => signup::draw_signup(f, app, chunks[1]),
            2 => login::draw_login(f, app, chunks[1]),
            _ => {}
        }
    }
}
