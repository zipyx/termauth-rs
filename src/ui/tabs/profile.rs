use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout, Alignment}, 
    widgets::{Block, Borders, BorderType, Paragraph}, 
    style::{Style, Color, Modifier}, text::{Span, Spans, Text}
};

use crate::{
    App,
    backend::service::user::{UserMode, Profile},
    component::block::centered_rect_a};
use super::utility::helper::draw_help_profile;

pub fn draw_profile<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(1),
                Constraint::Length(11),
            ].as_ref(),
        ).split(area);

    draw_profile_block(f, app, chunks[1]);
    draw_help_profile(f, app, chunks[2]);

}

fn draw_profile_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let old_password_pos = centered_rect_a(30, 8, f.size());
    let new_password_pos = Rect::new(old_password_pos.left(), old_password_pos.bottom(), old_password_pos.width, 3);
    let user_mode_pos = Rect::new(
        new_password_pos.left(), 
        new_password_pos.bottom(), 
        new_password_pos.width, 3);

    let error_message_pos = Rect::new(
        user_mode_pos.left(), 
        user_mode_pos.bottom(), 
        user_mode_pos.width, 3);

    let color_mode_error = Style::default().fg(Color::Red);

    // username block
    let color_mode_normal = Style::default().fg(Color::LightBlue);
    let color_mode_insert = Style::default().fg(Color::Yellow);

    let old_password_input = Paragraph::new(app.user.get_old_password())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Old Password"))
        .style(match app.user.get_profile_mode() {
            Profile::OldPassword => {
                match app.user.user_mode {
                    UserMode::OldPassword => color_mode_insert,
                    _ => color_mode_error,
                }
            }
            _ => Style::default(),
        });

    f.render_widget(old_password_input, old_password_pos);

    // password block
    let new_password_input = Paragraph::new(app.user.get_new_password())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("New Password"))
        .style(match app.user.get_profile_mode() {
            Profile::NewPassword => {
                match app.user.user_mode {
                    UserMode::NewPassword => color_mode_insert,
                    _ => color_mode_error,
                }
            }
            _ => Style::default(),
        });

    f.render_widget(new_password_input, new_password_pos);


    let (msg, style) = match app.user.user_mode {
        UserMode::Normal => (
            vec![
                Span::raw("You are now in "),
                Span::styled("Normal Mode", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue)),
            ],
            Style::default()
        ),

        UserMode::OldPassword => (
            vec![
                Span::raw("You are now in "),
                Span::styled("Insert Mode", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
            ],
            Style::default()
        ),

        UserMode::NewPassword => (
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
    let error_response = app.user.get_new_secure_password_error_message();
    let (error_message, error_message_style) = match app.user.user_mode {
        UserMode::Normal => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(error_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::OldPassword => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(error_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::NewPassword => (
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
    if app.user.get_new_secure_password_error_message().len() > 0 {
        f.render_widget(user_error_help_message, error_message_pos);
    }
}
