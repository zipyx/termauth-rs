use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout, Direction}, 
    widgets::{Block, Borders, ListItem, List}, 
    style::{Style, Modifier}, text::{Spans, Span}
};

use crate::App;
use super::utility::helper::draw_help_welcome;

pub fn draw_login<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

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

    draw_login_block(f, app, chunks[1]);
    draw_help_welcome(f, app, chunks[2]);
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
