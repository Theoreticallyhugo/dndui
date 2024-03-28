// use chrono;
// use std::process::Command;
use ratatui::{
    prelude::{Layout, Direction, Rect},
    layout::{Alignment, Constraint},
    style::{Color, Style},
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
        .split(area);

    frame.render_widget(
        Paragraph::new(format!(
            " {}  {}  {}  lvl {: >2}",
            app.character.get_name(), 
            app.character.get_race(), 
            app.character.get_class(), 
            app.character.get_level()
        ))
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
        Paragraph::new(format!(
            "strength\n{: >+2}\n{: >2}", 
            app.character.calculate_modifier(
                app.character.get_strength(), 
                false
            ), 
            app.character.get_strength()
        ))
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
        Paragraph::new(format!(
            "dexterity\n{: >+2}\n{: >2}", 
            app.character.calculate_modifier(
                app.character.get_dexterity(), 
                false
            ), 
            app.character.get_dexterity()
        ))
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
        Paragraph::new(format!(
            "constitution\n{: >+2}\n{: >2}", 
            app.character.calculate_modifier(
                app.character.get_constitution(), 
                false
            ), 
            app.character.get_constitution()
        ))
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
        Paragraph::new(format!(
            "intelligence\n{: >+2}\n{: >2}", 
            app.character.calculate_modifier(
                app.character.get_intelligence(), 
                false
            ), 
            app.character.get_intelligence()
        ))
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
        Paragraph::new(format!(
            "wisdom\n{: >+2}\n{: >2}", 
            app.character.calculate_modifier(
                app.character.get_wisdom(), 
                false
            ), 
            app.character.get_wisdom()
        ))
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
        Paragraph::new(format!(
            "charisma\n{: >+2}\n{: >2}", 
            app.character.calculate_modifier(
                app.character.get_charisma(), 
                false
            ), 
            app.character.get_charisma()
        ))
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
        Paragraph::new(format!("proficiency\n+{}\nbonus", app.character.get_proficiency()))
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
        Paragraph::new(format!(
            "walking\n  {: >3} ft.\nspeed", 
            app.character.get_walking_speed()
        ))
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
    let inspiration_colour = match app.get_input_mode() {
        InputMode::Inspiration => {
            Color::Red
        },
        _ => {
            Color::Cyan
        },
    };
    frame.render_widget(
        Paragraph::new(format!(
            "inspiration\n{}", 
            if app.character.get_inspiration() {
                // "󱍢\n░▒▓█▓▒░"
                // "\\\\//\n//\\\\"
                // "◊◊◊"
                // "/\\ /\\ /\\\n\\/ \\/ \\/"
                // "╳╳╳╳╳╳╳\n╳╳╳╳╳╳╳"
                // "⟪◊⟫\n     "
                // " \n "
                // "\n"
                "  \n  "
                // "  fancy \n  quote "
            } else {
                ""
            }
        ))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(inspiration_colour).bg(Color::Reset))
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

    let health_colour = match app.get_input_mode() {
        InputMode::Damaging => {
            Color::Red
        },
        _ => {
            Color::Cyan
        },
    };

    frame.render_widget(
        Paragraph::new(format!("heal\n{: >6}\ndamage", app.get_input()))
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
        Paragraph::new(format!("current\n{: >2}\nhp", app.character.get_current_hp()))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(health_colour).bg(Color::Reset))
        .centered(),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new(format!("max\n{: >2}\nhp", app.character.get_max_hp()))
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

    let temp_hp = if app.character.get_temp_hp() == 0 {
        "--".to_string()
    } else {
        app.character.get_temp_hp().to_string()
    };

    frame.render_widget(
        Paragraph::new(format!("temp\n{: >2}\nhp", temp_hp))
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
            format!("\n{} str: {: >+2}   {} int: {: >+2}\n\
            {} dex: {: >+2}   {} wis: {: >+2}\n\
            {} con: {: >+2}   {} cha: {: >+2}\n\n\
            saving throw modifiers", 
            if app.character.get_prof_str() {
                "●" 
            } else {
                "○" 
            },
            app.character.calculate_modifier(
                app.character.get_strength(), 
                app.character.get_prof_str()
            ),
            if app.character.get_prof_int() {
                "●" 
            } else {
                "○" 
            },
            app.character.calculate_modifier(
                app.character.get_intelligence(), 
                app.character.get_prof_int()
            ),
            if app.character.get_prof_dex() {
                "●" 
            } else {
                "○" 
            },
            app.character.calculate_modifier(
                app.character.get_dexterity(), 
                app.character.get_prof_dex()
            ),
            if app.character.get_prof_wis() {
                "●" 
            } else {
                "○" 
            },
            app.character.calculate_modifier(
                app.character.get_wisdom(), 
                app.character.get_prof_wis()
            ),
            if app.character.get_prof_con() {
                "●" 
            } else {
                "○" 
            },
            app.character.calculate_modifier(
                app.character.get_constitution(), 
                app.character.get_prof_con()
            ),
            if app.character.get_prof_cha() {
                "●" 
            } else {
                "○" 
            },
            app.character.calculate_modifier(
                app.character.get_charisma(), 
                app.character.get_prof_cha()
            ),
        )
        ) 
        .block(
            Block::bordered()
                .title(" saving throws ")
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
            format!("\n {: >2} passive wis (perception)\n \
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
                .title(" senses ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        layout[1]
    );

    // PROFICIECIES & LANGUAGES
    frame.render_widget(
        Paragraph::new(format!(
            "\n armor\n {}\n\n weapons\n {}\n\n tools\n {}\n\n languages\n {}",
            "Heavy Armor, Light Armor, \n Medium Armor, Shields",
            "Martial Weapons, Simple Weapons",
            "None",
            "Common, Draconic, Dwarvish, Elvish",
        )) 
        .block(
            Block::bordered()
                .title(" proficiencies & languages ")
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
        Paragraph::new( format!("\n\
            {} DEX Acrobatics      {: >+2}\n\
            {} WIS Animal Handling {: >+2}\n\
            {} INT Arcana          {: >+2}\n\
            {} STR Athletics       {: >+2}\n\
            {} CHA Deception       {: >+2}\n\
            {} INT History         {: >+2}\n\
            {} WIS Insight         {: >+2}\n\
            {} CHA Intimidation    {: >+2}\n\
            {} INT Investigation   {: >+2}\n\
            {} WIS Medicine        {: >+2}\n\
            {} INT Nature          {: >+2}\n\
            {} WIS Perception      {: >+2}\n\
            {} CHA Performance     {: >+2}\n\
            {} CHA Persuasion      {: >+2}\n\
            {} INT Religion        {: >+2}\n\
            {} DEX Sleight of Hand {: >+2}\n\
            {} DEX Stealth         {: >+2}\n\
            {} WIS Survival        {: >+2}\n\n\
            additional skills",
            app.get_dot(
                app.character.get_prof_acrobatics()
            ),
            app.character.calculate_modifier(
                app.character.get_dexterity(), 
                app.character.get_prof_acrobatics()
            ),
            app.get_dot(
                app.character.get_prof_animal_handling()
            ),
            app.character.calculate_modifier(
                app.character.get_wisdom(), 
                app.character.get_prof_animal_handling()
            ),
            app.get_dot(
                app.character.get_prof_arcana()
            ),
            app.character.calculate_modifier(
                app.character.get_intelligence(), 
                app.character.get_prof_arcana()
            ),
            app.get_dot(
                app.character.get_prof_athletics()
            ),
            app.character.calculate_modifier(
                app.character.get_strength(), 
                app.character.get_prof_athletics()
            ),
            app.get_dot(
                app.character.get_prof_deception()
            ),
            app.character.calculate_modifier(
                app.character.get_charisma(), 
                app.character.get_prof_deception()
            ),
            app.get_dot(
                app.character.get_prof_history()
            ),
            app.character.calculate_modifier(
                app.character.get_intelligence(), 
                app.character.get_prof_history()
            ),
            app.get_dot(
                app.character.get_prof_insight()
            ),
            app.character.calculate_modifier(
                app.character.get_wisdom(), 
                app.character.get_prof_insight()
            ),
            app.get_dot(
                app.character.get_prof_intimidation()
            ),
            app.character.calculate_modifier(
                app.character.get_charisma(), 
                app.character.get_prof_intimidation()
            ),
            app.get_dot(
                app.character.get_prof_investigation()
            ),
            app.character.calculate_modifier(
                app.character.get_intelligence(), 
                app.character.get_prof_investigation()
            ),
            app.get_dot(
                app.character.get_prof_medicine()
            ),
            app.character.calculate_modifier(
                app.character.get_wisdom(), 
                app.character.get_prof_medicine()
            ),
            app.get_dot(
                app.character.get_prof_nature()
            ),
            app.character.calculate_modifier(
                app.character.get_intelligence(), 
                app.character.get_prof_nature()
            ),
            app.get_dot(
                app.character.get_prof_perception()
            ),
            app.character.calculate_modifier(
                app.character.get_wisdom(), 
                app.character.get_prof_perception()
            ),
            app.get_dot(
                app.character.get_prof_performance()
            ),
            app.character.calculate_modifier(
                app.character.get_charisma(), 
                app.character.get_prof_performance()
            ),
            app.get_dot(
                app.character.get_prof_persuasion()
            ),
            app.character.calculate_modifier(
                app.character.get_charisma(), 
                app.character.get_prof_persuasion()
            ),
            app.get_dot(
                app.character.get_prof_religion()
            ),
            app.character.calculate_modifier(
                app.character.get_intelligence(), 
                app.character.get_prof_religion()
            ),
            app.get_dot(
                app.character.get_prof_sleight_of_hand()
            ),
            app.character.calculate_modifier(
                app.character.get_dexterity(), 
                app.character.get_prof_sleight_of_hand()
            ),
            app.get_dot(
                app.character.get_prof_stealth()
            ),
            app.character.calculate_modifier(
                app.character.get_dexterity(), 
                app.character.get_prof_stealth()
            ),
            app.get_dot(
                app.character.get_prof_survival()
            ),
            app.character.calculate_modifier(
                app.character.get_wisdom(), 
                app.character.get_prof_survival()
            ),
        )) 
        .block(
            Block::bordered()
                .title(" skills ")
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
        Paragraph::new(format!(
            "initiative\n{: >+2}", 
            app.character.get_initiative()
        )) 
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
        Paragraph::new(format!(
            "armor\n{}\nclass", 
            app.character.get_armor_class()
        )) 
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
            app.character.get_defenses()
        ) 
        .block(
            Block::bordered()
                .title(" defenses ")
                .title_alignment(Alignment::Center)
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
            app.character.get_conditions()
        ) 
        .block(
            Block::bordered()
                .title(" conditions ")
                .title_alignment(Alignment::Center)
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
                Press `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.get_counter()
        ))
        .block(
            Block::bordered()
                .title(" right ")
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
