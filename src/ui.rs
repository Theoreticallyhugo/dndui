// use chrono;
// use std::process::Command;
use ratatui::{
    prelude::{Layout, Direction, Rect},
    layout::{Alignment, Constraint},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::InputMode;


/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(5), Constraint::Fill(1)])
        .split(frame.size());

    headline(frame, app, layout[0]);
    first(frame, app, layout[1]);
// sudo powermetrics --samplers gpu_power -i500 -n 1 | grep 'active residency' | sed 's/[^0-9.%]//g' | sed 's/[%].*$//g'
    second(frame, app, layout[2]);
}

pub fn headline(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(format!(" {}  {}  {}  lvl {: >2}","name", "dragonborn", "paladin", "3"))
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new("short rest   long rest   edit ")
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .right_aligned(),
        layout[1],
    );
}
pub fn first(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::horizontal([
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Min(25),
    ])
    .split(area);

    first_str(frame, app, layout[0]);
    first_dex(frame, app, layout[1]);
    first_con(frame, app, layout[2]);
    first_int(frame, app, layout[3]);
    first_wis(frame, app, layout[4]);
    first_cha(frame, app, layout[5]);
    first_proficiency(frame, app, layout[6]);
    first_walking(frame, app, layout[7]);
    first_inspiration(frame, app, layout[8]);
    first_hp(frame, app, layout[9]);
}

pub fn first_str(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("strength\n{}\n{: >2}", "+3", 16))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_dex(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("dexterity\n{}\n{: >2}", "-1", 8))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_con(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("constitution\n{}\n{: >2}", "+2", 14))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_int(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("intelligence\n{}\n{: >2}", "-1", 8))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_wis(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("wisdom\n{}\n{: >2}", "+1", 12))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_cha(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("charisma\n{}\n{: >2}", "+3", 16))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_proficiency(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("proficiency\n{}\nbonus", "+2"))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_walking(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("walking\n  {: >3} ft.\nspeed", "30"))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_inspiration(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(format!("inspiration\n{}", ""))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area,
    );
}

pub fn first_hp(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(area);

    let mut health_colour: Color;
    match app.get_input_mode() {
        InputMode::Normal => {
            health_colour = Color::Cyan;
        },
        InputMode::Damaging => {
            health_colour = Color::Red;
        },
    }

    frame.render_widget(
        Paragraph::new("heal\n\ndamage")
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(health_colour).bg(Color::Reset))
        .centered(),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!("current\n{}\nhp", "28"))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new(format!("max\n{}\nhp", "28"))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        layout[2],
    );
    frame.render_widget(
        Paragraph::new(format!("temp\n{}\nhp", "--"))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        layout[3],
    );
}

pub fn second(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(50),
    ])
    .split(area);
    second_left(frame, app, layout[0]);
    second_middle(frame, app, layout[1]);
    second_right(frame, app, layout[2]);
}

pub fn second_left(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::vertical([
        Constraint::Min(7),
        Constraint::Min(7),
        Constraint::Percentage(60),
    ])
    .split(area);

    // SAVING THROWS
    frame.render_widget(
        Paragraph::new(
            format!("{} str: {}   {} int: {}\n\
            {} dex: {}   {} wis: {}\n\
            {} con: {}   {} cha: {}\n\n\
            saving throw modifiers", 
            "○", "+3",
            "○", "-1",
            "○", "-1",
            "●", "+3",
            "○", "+2",
            "●", "+5",
        )
        ) 
        .block(
            Block::bordered()
                .title("saving throws")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        layout[0]
    );

    // SENSES
    frame.render_widget(
        Paragraph::new(
            format!(" {: >2} passive wis (perception)\n \
            {: >2} passive int (investigation)\n \
            {: >2} passive wis (insight)\n\n \
            additional sense types", 
            11, 
            9, 
            11
        )
        ) 
        .block(
            Block::bordered()
                .title("senses")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        layout[1]
    );

    // PROFICIECIES & LANGUAGES
    frame.render_widget(
        Paragraph::new(
            format!("{} passive wis (perception)\n\
            {} passive int (investigation)\n\
            {} passive wis (insight)", 11, 9, 11)
        ) 
        .block(
            Block::bordered()
                .title("proficiecies & languages")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        layout[2]
    );
}

pub fn second_middle(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!("str: {}", 16)
        ) 
        .block(
            Block::bordered()
                .title("skills")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}

pub fn second_right(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::vertical([
        Constraint::Length(5),
        Constraint::Percentage(100),
    ])
    .split(area);
    second_right_top(frame, app, layout[0]);
    second_right_bot(frame, app, layout[1]);
}

pub fn second_right_top(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(35),
        Constraint::Percentage(35),
    ])
    .split(area);
    second_right_top_1(frame, app, layout[0]);
    second_right_top_2(frame, app, layout[1]);
    second_right_top_3(frame, app, layout[2]);
    second_right_top_4(frame, app, layout[3]);
}

pub fn second_right_top_1(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!("initiative\n{}", "-1")
        ) 
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}

pub fn second_right_top_2(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!("armor\n{}\nclass", 19)
        ) 
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}

pub fn second_right_top_3(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!("{} fire\n{} disease", "r", "i")
        ) 
        .block(
            Block::bordered()
                .title("defenses")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        area
    )
}

pub fn second_right_top_4(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "add active conditions"
        ) 
        .block(
            Block::bordered()
                .title("conditions")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        area
    )
}

pub fn second_right_bot(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title("right")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .centered(),
        area
    )
}

// pub fn second(frame: &mut Frame, app: &mut App, area: Rect) {
//     frame.render_widget(
//         Paragraph::new(format!(
//             "a"
//             // "current date: {}\ncurrent time: {}\ngpu: {}",
//             // chrono::offset::Local::now().date_naive().format("%d.%m.%Y"),
//             // chrono::offset::Local::now().time().format("%H:%M"),
//             // std::str::from_utf8(
//             // Command::new("sudo")
//             //     .arg("powermetrics --samplers gpu_power -i500 -n 1 | grep 'active residency' | sed 's/[^0-9.%]//g' | sed 's/[%].*$//g'")
//             //     .output()
//             //     .expect("cant do console stuff apparently? trying to get gpu stuff")
//             //     .stdout
//             //     .as_slice()
//             // ).expect("conversion from u8 to str failed for gpu usage")
//             // Command::new("echo")
//             //     .arg("\"hi\"")
//             //     .output()
//             //     .expect("cant do console stuff apparently? trying to get gpu stuff")
//             //     .stdout
//             //     .as_slice()
//             // ).expect("conversion from u8 to str failed for gpu usage")
//                 // FIXME: convert this properly
//         ))
//         .block(
//             Block::bordered()
//                 .title("chrono")
//                 .title_alignment(Alignment::Center)
//                 .border_type(BorderType::Rounded),
//         )
//         .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
//         .centered(),
//         area,
//     );
// }
