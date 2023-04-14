use crate::component::block::centered_rect_a;
use crate::App;

use tui::{
    backend::Backend, 
    Frame, 
    layout::{Layout, Constraint, Alignment}, 
    widgets::{Paragraph, Wrap, Block, Borders, Clear}, 
    style::{Style, Color}, 
    text::Span
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {

    let size = f.size();
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
        .split(size);

    // Some random text to add
    let text = if app.signup {
        "Sign Up"
    } else {
        "Login"
    };

    // Text component (large)
    let paragraph = Paragraph::new(Span::styled(
        text,
        Style::default()
    )).alignment(Alignment::Center).wrap(Wrap { trim: true });

    // Render first block
    f.render_widget(paragraph, chunks[0]);

    // Create block with title and color for data access
    let block = Block::default()
        .title("Data Access")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    // Render second block
    f.render_widget(block, chunks[1]);

    // Display popup if app selection is signup
    if app.signup {
        let block = Block::default()
            .title("Sign Up")
            .borders(Borders::ALL);
        let area = centered_rect_a(40, 20, size);
        // Render the popup
        f.render_widget(Clear, area);
        f.render_widget(block, area);
    }
}
