use tui::{
    backend::Backend, 
    Frame, 
    layout::{Rect, Layout, Direction, Constraint}, 
    widgets::{
        Paragraph, 
        Block, 
        Borders,
        List,
        ListItem, 
    }, 
    style::{ Style, Color, Modifier }, 
    text::{Span, Spans}};
use crate::{App, component::block::centered_rect_a};


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App, prompt: &str, input: &String, user_choice: usize) {

    let size = f.size();
    // two different areas that will be rendered
    let main_area = centered_rect_a(30, 14, size);
    let info_area = Rect::new(main_area.left(), main_area.bottom() + 1, main_area.width, 8);

    // if the signup is true, render the signup block
    if app.signup {

        // character chunks for the input block
        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .margin(2)
        //     .constraints([Constraint::Length(3)].as_ref())
        //     .split(size);

        // input block
        let input_block = Paragraph::new(format!("{}{}", prompt, input))
            .block(Block::default()
            .borders(Borders::ALL)
            .title("Sign Up"));

        // let paragraph_block = Paragraph::new("Hello, World!")
        //     .block(Block::default()
        //     .borders(Borders::ALL)
        //     .title("Welcome!"));
        // f.render_widget(paragraph_block, main_area);

        f.render_widget(input_block, main_area);

    // else show the menu
    } else {

        let menu_items = ["Username:", "Password:"]
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
            .block(Block::default().borders(Borders::ALL).title("Sign Up"))
            .highlight_style(Style::default().bg(Color::Green));

        // render menu in the main area
        f.render_widget(menu, main_area);

        // character chunks for the input block
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3)].as_ref())
            .split(size);

        // input block
        let input_block = Paragraph::new(format!("{}{}", prompt, input))
            .block(Block::default()
            .borders(Borders::ALL)
            .title("Sign Up"));

        f.render_widget(input_block, chunks[0]);

        // render information text for menu
        let menu_text = vec![
            Spans::from(vec![
                Span::styled("i: ", Style::default().fg(Color::Yellow)),
                Span::from("insert mode"),
            ]),
            Spans::from(vec![
                Span::styled("esc: ", Style::default().fg(Color::Yellow)),
                Span::from("exit insert mode into normal mode"),
            ]),
        ];

        // build help block containing title and color
        let help_block = Paragraph::new(menu_text)
            .block(Block::default().borders(Borders::ALL).title("Help keys"));

        // Render help block with the help text
        f.render_widget(help_block, info_area);
    }
}
