use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout, Alignment}, 
    widgets::{Block, Borders, BorderType, Paragraph}, 
    style::{Style, Color, Modifier}, text::{Span, Spans, Text}
};

use crate::{
    App,
    backend::service::user::{Login, UserMode},
    component::block::centered_rect_a};
use super::utility::helper::draw_help_login;

pub fn draw_login<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(1),
                Constraint::Length(11),
            ].as_ref(),
        ).split(area);

    // Another appropriate layout if the other isn't adaptable
    // let chunks = Layout::default()
    //     .constraints(
    //         [
    //             Constraint::Length(9),
    //             Constraint::Min(8),
    //             Constraint::Length(7),
    //         ].as_ref(),
    //     ).split(area);

    draw_login_block(f, app, chunks[1]);
    draw_help_login(f, app, chunks[2]);

}

fn draw_login_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let username_pos = centered_rect_a(30, 8, f.size());
    let password_pos = Rect::new(username_pos.left(), username_pos.bottom(), username_pos.width, 3);
    let user_mode_pos = Rect::new(
        password_pos.left(), 
        password_pos.bottom(), 
        password_pos.width, 3);

    let error_message_pos = Rect::new(
        user_mode_pos.left(), 
        user_mode_pos.bottom(), 
        user_mode_pos.width, 3);

    let color_mode_error = Style::default().fg(Color::Red);

    // username block
    let color_mode_normal = Style::default().fg(Color::LightBlue);
    let color_mode_insert = Style::default().fg(Color::Yellow);

    let username_input = Paragraph::new(app.user.get_login_username())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Username"))
        .style(match app.user.get_login_mode() {
            Login::Username => {
                match app.user.user_mode {
                    UserMode::Username => color_mode_insert,
                    _ => color_mode_normal,
                }
            }
            _ => Style::default(),
        });

    f.render_widget(username_input, username_pos);

    // password block
    let password_input = Paragraph::new(app.user.get_login_password())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Password"))
        .style(match app.user.get_login_mode() {
            Login::Password => {
                match app.user.user_mode {
                    UserMode::Password => color_mode_insert,
                    _ => color_mode_normal,
                }
            }
            _ => Style::default(),
        });

    f.render_widget(password_input, password_pos);


    let (msg, style) = match app.user.user_mode {
        UserMode::Normal => (
            vec![
                Span::raw("You are now in "),
                Span::styled("Normal Mode", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue)),
            ],
            Style::default()
        ),

        UserMode::Username => (
            vec![
                Span::raw("You are now in "),
                Span::styled("Insert Mode", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
            ],
            Style::default()
        ),

        UserMode::Password => (
            vec![
                Span::raw("You are now in "),
                Span::styled("Insert Mode", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
            ],
            Style::default()
        ),
        _ => (vec![], Style::default()),
    };

    let mut user_mode = Text::from(Spans::from(msg.to_owned()));
    user_mode.patch_style(style);
    let help_message = Paragraph::new(user_mode).alignment(Alignment::Center);
    f.render_widget(help_message, user_mode_pos);

    // Error blinkers
    let error_response = app.user.get_signup_username_error_message();
    let (error_message, error_message_style) = match app.user.user_mode {
        UserMode::Normal => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(error_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Username => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(error_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Password => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(error_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        _ => (vec![], Style::default()),
    };

    let mut user_error = Text::from(Spans::from(error_message.to_owned()));
    user_error.patch_style(error_message_style);

    let user_error_help_message = Paragraph::new(user_error).alignment(Alignment::Center);

}
