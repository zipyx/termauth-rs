use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout}, 
    widgets::{Paragraph, Block, Borders, BorderType}, 
    style::{Style, Color}
};
use crate::{App, component::block::centered_rect_a, 
    backend::service::user::CredentialManager
};
use super::utility::helper::draw_help_welcome;

pub fn draw_credential_manager<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(1),
                Constraint::Length(11),
            ].as_ref(),
        ).split(area);

    // let chunks = Layout::default()
    //     .constraints(
    //         [
    //             Constraint::Length(9),
    //             Constraint::Min(8),
    //             Constraint::Length(7),
    //         ].as_ref(),
    //     ).split(area);

    // draw_help_popup(f, app, chunks[2]);
    draw_credential_manager_block(f, app, chunks[1]);
    draw_help_welcome(f, app, chunks[2]);

}

fn draw_credential_manager_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

    let app_pos = centered_rect_a(40, 8, f.size());
    let username_pos = Rect::new(app_pos.left(), app_pos.bottom(), app_pos.width, 4);
    let password_pos = Rect::new(username_pos.left(), username_pos.bottom(), username_pos.width, 4);

    let app_input = Paragraph::new(app.user.app_name.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("App"))
        .style( match app.user.app {
                CredentialManager::App => Style::default().fg(Color::Magenta),
                _ => Style::default(),
        });

    f.render_widget(app_input, app_pos);

    let username_input = Paragraph::new(app.user.app_username.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Username"))
        .style(match app.user.app {
            CredentialManager::Username => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        });

    f.render_widget(username_input, username_pos);

    let password_input = Paragraph::new(app.user.app_password.as_ref())
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Password"))
        .style(match app.user.app {
            CredentialManager::Password => Style::default().fg(Color::Magenta),
            _ => Style::default(),
        });

    f.render_widget(password_input, password_pos);

}
