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
use crate::help_ui::help_screen;
use crate::tabs_ui::second_right_bot;


/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    
    if app.get_help_screen_shown() {
        help_screen(frame, app, frame.size());
    } else {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(5), Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.size());

        headline(frame, app, layout[0]);
        first(frame, app, layout[1]);
        second(frame, app, layout[2]);
        colon_line(frame, app, layout[3]);
    }

}

pub fn headline(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    frame.render_widget(
        Paragraph::new(format!(
            " {}  {}  {}  lvl {: >2}",
            app.character.name(), 
            app.character.race(), 
            app.character.class(), 
            app.character.level()
        ))
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        layout[0],
    );

    // TODO: r for short rest, R for long. highlight respective word and press again to lock in
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
            app.character.strength().modifier, 
            app.character.strength().score
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
            app.character.dexterity().modifier, 
            app.character.dexterity().score
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
            app.character.constitution().modifier, 
            app.character.constitution().score
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
            app.character.intelligence().modifier, 
            app.character.intelligence().score
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
            app.character.wisdom().modifier, 
            app.character.wisdom().score
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
            app.character.charisma().modifier, 
            app.character.charisma().score
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
        Paragraph::new(format!("proficiency\n+{}\nbonus", app.character.proficiency()))
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
            app.character.walking_speed()
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
            if app.character.inspiration() {
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

    // let current_colour = match app.get_input_mode() {
    //     InputMode::Healing => {
    //         Color::Red
    //     },
    //     _ => {
    //         Color::Cyan
    //     },
    // };
    //
    let heal_damage_colour = match app.get_input_mode() {
        InputMode::Healing | InputMode::TempHealing => {
            Color::Red
        },
        _ => {
            Color::Cyan
        },
    };

    let current_colour = match app.get_input_mode() {
        InputMode::Healing => {
            Color::Red
        },
        _ => {
            Color::Cyan
        },
    };

    let temp_colour = match app.get_input_mode() {
        InputMode::TempHealing => {
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
        .style(Style::default().fg(heal_damage_colour).bg(Color::Reset))
        .centered(),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!("current\n{: >2}\nhp", app.character.current_hp()))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(current_colour).bg(Color::Reset))
        .centered(),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new(format!("max\n{: >2}\nhp", app.character.max_hp()))
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

    let temp_hp = if app.character.temp_hp() == 0 {
        "--".to_string()
    } else {
        app.character.temp_hp().to_string()
    };

    frame.render_widget(
        Paragraph::new(format!("temp\n{: >+2}\nhp", temp_hp))
        .block(
            Block::bordered()
                .title("")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(temp_colour).bg(Color::Reset))
        .centered(),
        layout[3],
    );
}

pub fn second(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::horizontal([
        Constraint::Length(35),
        Constraint::Length(35),
        Constraint::Percentage(100),
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
            if app.character.strength().proficiency {
                "●" 
            } else {
                "○" 
            },
            app.character.strength().save, 
            if app.character.intelligence().proficiency {
                "●" 
            } else {
                "○" 
            },
            app.character.intelligence().save, 
            if app.character.dexterity().proficiency {
                "●" 
            } else {
                "○" 
            },
            app.character.dexterity().save, 
            if app.character.wisdom().proficiency {
                "●" 
            } else {
                "○" 
            },
            app.character.wisdom().save, 
            if app.character.constitution().proficiency {
                "●" 
            } else {
                "○" 
            },
            app.character.constitution().save, 
            if app.character.charisma().proficiency {
                "●" 
            } else {
                "○" 
            },
            app.character.charisma().save, 
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
            app.character.passive_perception(), 
            app.character.passive_investigation(), 
            app.character.passive_insight(), 
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
            "Common, Draconic, Dwarvish,\n Elvish",
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
            {} DEX Acrobatics      {: >1} {: >+2}\n\
            {} WIS Animal Handling {: >1} {: >+2}\n\
            {} INT Arcana          {: >1} {: >+2}\n\
            {} STR Athletics       {: >1} {: >+2}\n\
            {} CHA Deception       {: >1} {: >+2}\n\
            {} INT History         {: >1} {: >+2}\n\
            {} WIS Insight         {: >1} {: >+2}\n\
            {} CHA Intimidation    {: >1} {: >+2}\n\
            {} INT Investigation   {: >1} {: >+2}\n\
            {} WIS Medicine        {: >1} {: >+2}\n\
            {} INT Nature          {: >1} {: >+2}\n\
            {} WIS Perception      {: >1} {: >+2}\n\
            {} CHA Performance     {: >1} {: >+2}\n\
            {} CHA Persuasion      {: >1} {: >+2}\n\
            {} INT Religion        {: >1} {: >+2}\n\
            {} DEX Sleight of Hand {: >1} {: >+2}\n\
            {} DEX Stealth         {: >1} {: >+2}\n\
            {} WIS Survival        {: >1} {: >+2}\n\n\
            additional skills",
            app.get_dot(
                app.character.acrobatics().proficiency
            ),
            app.get_advantage_letter(app.character.acrobatics().advantage),
            app.character.acrobatics().modifier,
            app.get_dot(
                app.character.animal_handling().proficiency
            ),
            app.get_advantage_letter(app.character.animal_handling().advantage),
            app.character.animal_handling().modifier,
            app.get_dot(
                app.character.arcana().proficiency
            ),
            app.get_advantage_letter(app.character.arcana().advantage),
            app.character.arcana().modifier,
            app.get_dot(
                app.character.athletics().proficiency
            ),
            app.get_advantage_letter(app.character.athletics().advantage),
            app.character.athletics().modifier,
            app.get_dot(
                app.character.deception().proficiency
            ),
            app.get_advantage_letter(app.character.deception().advantage),
            app.character.deception().modifier,
            app.get_dot(
                app.character.history().proficiency
            ),
            app.get_advantage_letter(app.character.history().advantage),
            app.character.history().modifier,
            app.get_dot(
                app.character.insight().proficiency
            ),
            app.get_advantage_letter(app.character.insight().advantage),
            app.character.insight().modifier,
            app.get_dot(
                app.character.intimidation().proficiency
            ),
            app.get_advantage_letter(app.character.intimidation().advantage),
            app.character.intimidation().modifier,
            app.get_dot(
                app.character.investigation().proficiency
            ),
            app.get_advantage_letter(app.character.investigation().advantage),
            app.character.investigation().modifier,
            app.get_dot(
                app.character.medicine().proficiency
            ),
            app.get_advantage_letter(app.character.medicine().advantage),
            app.character.medicine().modifier,
            app.get_dot(
                app.character.nature().proficiency
            ),
            app.get_advantage_letter(app.character.nature().advantage),
            app.character.nature().modifier,
            app.get_dot(
                app.character.perception().proficiency
            ),
            app.get_advantage_letter(app.character.perception().advantage),
            app.character.perception().modifier,
            app.get_dot(
                app.character.performance().proficiency
            ),
            app.get_advantage_letter(app.character.performance().advantage),
            app.character.performance().modifier,
            app.get_dot(
                app.character.persuasion().proficiency
            ),
            app.get_advantage_letter(app.character.persuasion().advantage),
            app.character.persuasion().modifier,
            app.get_dot(
                app.character.religion().proficiency
            ),
            app.get_advantage_letter(app.character.religion().advantage),
            app.character.religion().modifier,
            app.get_dot(
                app.character.sleight_of_hand().proficiency
            ),
            app.get_advantage_letter(app.character.sleight_of_hand().advantage),
            app.character.sleight_of_hand().modifier,
            app.get_dot(
                app.character.stealth().proficiency
            ),
            app.get_advantage_letter(app.character.stealth().advantage),
            app.character.stealth().modifier,
            app.get_dot(
                app.character.survival().proficiency
            ),
            app.get_advantage_letter(app.character.survival().advantage),
            app.character.survival().modifier,
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
        Constraint::Length(14),
        Constraint::Length(11),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
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
            app.character.initiative()
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
            app.character.armor_class()
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
            app.character.defenses()
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
            app.character.conditions()
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


pub fn colon_line(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(
        Paragraph::new(" press : for commands and ? for help")
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
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
