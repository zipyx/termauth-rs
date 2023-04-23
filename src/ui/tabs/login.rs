use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout}, 
    widgets::{Block, Borders, BorderType, Paragraph}, 
    style::{Style, Color}
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

    // username block
    let color_mode_normal = Style::default().fg(Color::LightBlue);
    let color_mode_insert = Style::default().fg(Color::Yellow);

    let username_input = Paragraph::new(app.user.login_username.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Username"))
        .style(match app.user.login {
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
    let password_input = Paragraph::new(app.user.login_password.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Password"))
        .style(match app.user.login {
            Login::Password => {
                match app.user.user_mode {
                    UserMode::Password => color_mode_insert,
                    _ => color_mode_normal,
                }
            }
            _ => Style::default(),
        });

    f.render_widget(password_input, password_pos);

}
