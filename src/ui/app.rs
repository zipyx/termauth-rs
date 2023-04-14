use tui::{
    backend::Backend, 
    Frame, 
    layout::{Rect, Layout, Constraint}, 
    widgets::{
        Paragraph, 
        Block, 
        Borders, 
        ListItem, 
        List
    }, 
    style::{ Style, Modifier, Color }, 
    text::{Span, Spans}};

use crate::{App, component::block::centered_rect_a};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app;
}
