use tui::{backend::Backend, Frame, 
    layout::{Rect, Constraint, Layout, Direction, Alignment}, 
    widgets::{Paragraph, Wrap},
    style::{Style, Color}
};

use crate::{
    App,
    backend::service::utility::constants::ENROLMENT_INSTRUCTIONS,
};
use super::utility::helper::draw_help_welcome;

pub fn draw_welcome<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

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

    draw_welcome_block(f, app, chunks[1]);
    draw_help_welcome(f, app, chunks[2]);
}

fn draw_welcome_block<B: Backend>(f: &mut Frame<B>,app: &mut App, area: Rect) {

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(100),
            ].as_ref(),
        ).split(area);

    {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(100),
                ].as_ref(),
            ).direction(Direction::Horizontal) .split(chunks[0]);

        let instruction = Paragraph::new(ENROLMENT_INSTRUCTIONS)
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left)
            // .wrap(Wrap { trim: true })
            .scroll((app.scroll, 0));

        f.render_widget(instruction, chunks[0]);
    }
}
