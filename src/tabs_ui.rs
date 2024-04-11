use ratatui::{
    // prelude::{Layout, Direction, Rect},
    prelude::Rect,
    // layout::{Alignment, Constraint},
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;
// use crate::app::InputMode;
use crate::character_limbs::spells::Spell;


pub fn second_right_bot(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.get_counter() {
        0 => actions(frame, app, area),
        1 => spells(frame, app, area),
        2 => inventory(frame, app, area),
        3 => features_traits(frame, app, area),
        4 => description(frame, app, area),
        5 => notes(frame, app, area),
        6 => extras(frame, app, area),
        _ => {}
    }
    // frame.render_widget(
    //     Paragraph::new(
    //         format!(
    //         "This is a tui template.\n\
    //             Press `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //         app.get_counter()
    //     ))
    //     .block(
    //         Block::bordered()
    //             .title(" right ")
    //             .title_alignment(Alignment::Center)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
    //     .centered(),
    //     area
    // )
}

pub fn actions(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!(
            "This is a tui template.\n\
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title(" actions ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}
pub fn pretty_spells(spells: &[Spell]) -> String {
    let mut pretty_out = "\n NAME                               TIME    RANGE  HIT/ DC      EFFECT   NOTES".to_string();
    for spell in spells.iter() {
        pretty_out.extend(format!(
            //   name   conc  rit    time  Fmt      rng,  hit,    eff, dur, aoeFmt, comp, cls  
            "\n {:.<30} {: >1}{: >1} {: >2}{: <3}   {: >6}  {: >7}  {: >10}  {}{} {}, {}",
            spell.name,
            if spell.concentration {
                "C"
            } else {
                ""
            },
            if spell.ritual {
                "R"
            } else {
                ""
            },
            spell.time,
            spell.time_format,
            if spell.range == 0 {
                "touch".to_string()
            }else {
                format!("{}ft.", spell.range)
            },
            if spell.hit_dc == 0 {
                "----- ".to_string()
            } else {
                format!("{}: {: >2}", spell.hit_dc_ability, spell.hit_dc)
            },
            spell.effect,
            if spell.duration == 0 {
                "".to_string()
            } else {
                format!(" D: {}{},", spell.duration, spell.duration_format)
            },
            if spell.aoe == 0 {
                "".to_string()
            } else {
                format!(" {}ft. {},", spell.aoe, spell.aoe_format)
            },
            spell.components,
            spell.class,
        ).chars());
    }
    pretty_out
}
pub fn spells(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(pretty_spells(app.character.spells_prepared()))
        .block(
            Block::bordered()
                .title(" spells ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        area
    )
}
pub fn inventory(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!(
            "This is a tui template.\n\
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title(" inventory ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}
pub fn features_traits(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!(
            "This is a tui template.\n\
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title(" features & traits ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}
pub fn description(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!(
            "This is a tui template.\n\
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title(" description ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}
pub fn notes(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!(
            "This is a tui template.\n\
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title(" notes ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}
pub fn extras(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!(
            "This is a tui template.\n\
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title(" extras ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}
