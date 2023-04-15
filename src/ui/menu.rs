// use tui::{
//     backend::Backend, 
//     Frame, 
//     layout::Rect, 
//     widgets::{
//         Paragraph, 
//         Block, 
//         Borders, 
//         ListItem, 
//         List
//     }, 
//     style::{ Style, Modifier, Color }, 
//     text::{Span, Spans}};
// use crate::{App, component::block::centered_rect_a};

// pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App, user_choice: usize) {
//     let size = f.size();

//     // prompt
//     let prompt = ">> ";
//     let mut input = String::new();

//     // two different areas that will be rendered
//     let main_area = centered_rect_a(20, 14, size);
//     let info_area = Rect::new(main_area.left(), main_area.bottom() + 1, main_area.width, 8);

//     // if the signup is true, render the signup block
//     if app.signup {
//         let paragraph_block = Paragraph::new("Hello, World!")
//             .block(Block::default()
//             .borders(Borders::ALL)
//             .title("Welcome!"));
//         f.render_widget(paragraph_block, main_area);
//         // signup::ui(f, app, prompt, &input, user_choice);

//     // else show the menu
//     } else {
//         let menu_items = ["Sign Up", "Login", "Quit"]
//             .iter()
//             .enumerate()
//             .map(|(i, &item)| {
//                 if i == user_choice {
//                     ListItem::new(Span::styled(
//                         item,
//                         Style::default().add_modifier(Modifier::REVERSED),
//                     ))
//                 } else {
//                     ListItem::new(item)
//                 }
//             }).collect::<Vec<ListItem>>();

//         // create menu block with title and color
//         let menu = List::new(menu_items)
//             .block(Block::default().borders(Borders::ALL).title("Menu"))
//             .highlight_style(Style::default().fg(Color::Green));

//         // render menu in the main area
//         f.render_widget(menu, main_area);

//         // render information text for menu
//         let menu_text = vec![
//             Spans::from(vec![
//                 Span::styled("k: ", Style::default().fg(Color::Yellow)),
//                 Span::from("Move cursor up"),
//             ]),
//             Spans::from(vec![
//                 Span::styled("j: ", Style::default().fg(Color::Yellow)),
//                 Span::from("Move cursor down"),
//             ]),
//             Spans::from(vec![
//                 Span::styled("a: ", Style::default().fg(Color::Yellow)),
//                 Span::from("toggle menu on/off"),
//             ]),
//             Spans::from(vec![
//                 Span::styled("q: ", Style::default().fg(Color::Yellow)),
//                 Span::from("Quit"),
//             ]),
//         ];

//         // build help block containing title and color
//         let help_block = Paragraph::new(menu_text)
//             .block(Block::default().borders(Borders::ALL).title("Help keys"));

//         // Render help block with the help text
//         f.render_widget(help_block, info_area);
//     }
// }
