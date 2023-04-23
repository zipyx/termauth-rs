use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout, Alignment}, 
    widgets::{Block, Borders, BorderType, Paragraph}, 
    style::{Style, Color, Modifier}, text::{Span, Text, Spans},
};

use crate::{ App,
    backend::service::user::{SignUp, UserMode},
    component::block::centered_rect_a, ui::app};
use super::utility::helper::draw_help_signup;

pub fn draw_signup<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

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

    draw_signup_block(f, app, chunks[1]);
    draw_help_signup(f, app, chunks[2]);

}

fn draw_signup_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let username_pos = centered_rect_a(30, 8, f.size());
    let password_pos = Rect::new(username_pos.left(), username_pos.bottom(), username_pos.width, 3);

    let user_mode_pos = Rect::new(
        password_pos.left(), 
        password_pos.bottom(), 
        password_pos.width, 3);

    let username_error_message_pos = Rect::new(
        user_mode_pos.left(), 
        user_mode_pos.bottom(), 
        user_mode_pos.width, 3);

    let password_error_message_pos = Rect::new(
        username_error_message_pos.left(), 
        username_error_message_pos.bottom(), 
        username_error_message_pos.width, 3);

    // Color palette modes
    let color_mode_normal = Style::default().fg(Color::LightBlue);
    let color_mode_insert = Style::default().fg(Color::Yellow);
    let color_mode_error = Style::default().fg(Color::Red);

    let username_input = Paragraph::new(app.user.get_signup_username())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Username"))
        .style(match app.user.signup {
            SignUp::Username => {
                match app.user.user_mode {
                    UserMode::Username => color_mode_insert,
                    _ => color_mode_normal,
                }
            }
            _ => Style::default(),
        });

    f.render_widget(username_input, username_pos);

    // password block
    let password_input = Paragraph::new(app.user.get_signup_password())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Password"))
        .style(match app.user.signup {
            SignUp::Password => {
                match app.user.user_mode {
                    UserMode::Password => color_mode_insert,
                    _ => color_mode_normal,
                }
            }
            _ => Style::default(),
        });

    f.render_widget(password_input, password_pos);

    // Username error blocks - optional
    let _username_error_message = Paragraph::new(app.user.get_signup_username_error_message())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Error Response")
            .title_alignment(tui::layout::Alignment::Left))
        .style(match app.user.signup {
            SignUp::Username => {
                match app.user.user_mode {
                    UserMode::Normal => color_mode_error,
                    _ => color_mode_error,
                }
            }
            SignUp::Password => {
                match app.user.user_mode {
                    UserMode::Normal => color_mode_error,
                    _ => color_mode_error,
                }
            }
            // _ => Style::default(),
    });

    // Password error blocks - optional
    let _password_error_message = Paragraph::new(app.user.get_signup_password_error_message())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Error Response")
            .title_alignment(tui::layout::Alignment::Left))
        .style(match app.user.signup {
            SignUp::Password => {
                match app.user.user_mode {
                    UserMode::Normal => color_mode_error,
                    _ => color_mode_error,
                }
            }
            SignUp::Username => {
                match app.user.user_mode {
                    UserMode::Normal => color_mode_error,
                    _ => color_mode_error,
                }
            }
            // _ => Style::default(),
    });

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

    let username_response = app.user.get_signup_username_error_message();
    let password_response = app.user.get_signup_password_error_message();

    let (error_username, error_username_style) = match app.user.user_mode {
        UserMode::Normal => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(username_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Username => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(username_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Password => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(username_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        _ => (vec![], Style::default()),
    };

    let (error_password, error_password_style) = match app.user.user_mode {
        UserMode::Normal => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(password_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Username => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(password_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        UserMode::Password => (
            vec![
                Span::styled("System Error: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::styled(password_response, Style::default().fg(Color::Red)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        _ => (vec![], Style::default()),
    };

    let mut user_error = Text::from(Spans::from(error_username.to_owned()));
    let mut pass_error = Text::from(Spans::from(error_password.to_owned()));

    user_error.patch_style(error_username_style);
    pass_error.patch_style(error_password_style);

    let user_error_help_message = Paragraph::new(user_error).alignment(Alignment::Center);
    let pass_error_help_message = Paragraph::new(pass_error).alignment(Alignment::Center);

    if app.user.get_signup_username_error_message().len() > 0 {
        f.render_widget(user_error_help_message, username_error_message_pos);
    }

    if app.user.get_signup_password_error_message().len() > 0 {
        f.render_widget(pass_error_help_message, password_error_message_pos);
    }

    // if app.user.get_signup_password_error_message().len() > 0 {
    //     f.render_widget(password_error_message, password_error_message_pos);
    // }

}
