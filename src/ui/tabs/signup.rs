use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout}, 
    widgets::{Block, Borders, BorderType, Paragraph}, 
    style::{Style, Color}
};

use crate::{
    App,
    backend::service::user::SignUp,
    component::block::centered_rect_a};
use super::utility::helper::draw_help_welcome;

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
    draw_help_welcome(f, app, chunks[2]);

}

fn draw_signup_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let username_pos = centered_rect_a(30, 8, f.size());
    let password_pos = Rect::new(username_pos.left(), username_pos.bottom(), username_pos.width, 3);

    // username block
    let username_input = Paragraph::new(app.new_username.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Username"))
        .style(match app.signup {
            SignUp::Username => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

    f.render_widget(username_input, username_pos);

    // password block
    let password_input = Paragraph::new(app.new_password.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Password"))
        .style(match app.signup {
            SignUp::Password => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

    f.render_widget(password_input, password_pos);

}
