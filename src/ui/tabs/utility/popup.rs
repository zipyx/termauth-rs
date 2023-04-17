use tui::{
    backend::Backend, 
    Frame, 
    layout::Rect, 
    widgets::{Paragraph, Block, Borders, ListItem, List}, 
    text::{Span, Spans}, 
    style::{Style, Modifier, Color}
};

use crate::{
    App, 
    component::block::centered_rect_a,
    backend::service::utility::constants::VISITOR,
};


pub fn _draw_popup_hello_world<B: Backend>(f: &mut Frame<B>, _app: &mut App, _area: Rect) {

    let size = f.size();
    let main_area = centered_rect_a(20, 14, size);

    let paragraph_block = Paragraph::new("Hello, World!")
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Welcome!"));
    f.render_widget(paragraph_block, main_area);
}


pub fn _draw_popup_menu<B: Backend>(f: &mut Frame<B>, _app: &mut App, _area: Rect) {

    let user_choice = 0;
    let size = f.size();
    let main_area = centered_rect_a(20, 14, size);
    let info_area = Rect::new(main_area.left(), main_area.bottom() + 1, main_area.width, 8);
    let menu_items = VISITOR
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
