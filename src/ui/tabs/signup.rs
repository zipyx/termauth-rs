use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout}, 
    widgets::{Block, Borders, BorderType, Paragraph}, 
    style::{Style, Color},
};

use crate::{ App,
    backend::service::user::{SignUp, UserMode},
    component::block::centered_rect_a};
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
    let error_message_pos = Rect::new(password_pos.left(), password_pos.bottom(), username_pos.width, 3);

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

    let error_message = Paragraph::new(app.user.error_message.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Error Response").title_alignment(tui::layout::Alignment::Center))
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

    if app.user.error_message.len() > 0 {
        f.render_widget(error_message, error_message_pos);
    }

}
