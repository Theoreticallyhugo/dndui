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
            app.character.get_name(), 
            app.character.get_race(), 
            app.character.get_class(), 
            app.character.get_level()
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
            app.character.get_strength().2, 
            app.character.get_strength().0
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
            app.character.get_dexterity().2, 
            app.character.get_dexterity().0
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
            app.character.get_constitution().2, 
            app.character.get_constitution().0
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
            app.character.get_intelligence().2, 
            app.character.get_intelligence().0
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
            app.character.get_wisdom().2, 
            app.character.get_wisdom().0
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
            app.character.get_charisma().2, 
            app.character.get_charisma().0
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
        Paragraph::new(format!("current\n{: >2}\nhp", app.character.get_current_hp()))
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
            if app.character.get_strength().1 {
                "●" 
            } else {
                "○" 
            },
            app.character.get_strength().3, 
            if app.character.get_intelligence().1 {
                "●" 
            } else {
                "○" 
            },
            app.character.get_intelligence().3, 
            if app.character.get_dexterity().1 {
                "●" 
            } else {
                "○" 
            },
            app.character.get_dexterity().3, 
            if app.character.get_wisdom().1 {
                "●" 
            } else {
                "○" 
            },
            app.character.get_wisdom().3, 
            if app.character.get_constitution().1 {
                "●" 
            } else {
                "○" 
            },
            app.character.get_constitution().3, 
            if app.character.get_charisma().1 {
                "●" 
            } else {
                "○" 
            },
            app.character.get_charisma().3, 
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
            app.character.get_passive_perception(), 
            app.character.get_passive_investigation(), 
            app.character.get_passive_insight(), 
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
                app.character.get_acrobatics().0
            ),
            app.get_advantage_letter(&app.character.get_acrobatics().2),
            app.character.get_acrobatics().1,
            app.get_dot(
                app.character.get_animal_handling().0
            ),
            app.get_advantage_letter(&app.character.get_animal_handling().2),
            app.character.get_animal_handling().1,
            app.get_dot(
                app.character.get_arcana().0
            ),
            app.get_advantage_letter(&app.character.get_arcana().2),
            app.character.get_arcana().1,
            app.get_dot(
                app.character.get_athletics().0
            ),
            app.get_advantage_letter(&app.character.get_athletics().2),
            app.character.get_athletics().1,
            app.get_dot(
                app.character.get_deception().0
            ),
            app.get_advantage_letter(&app.character.get_deception().2),
            app.character.get_deception().1,
            app.get_dot(
                app.character.get_history().0
            ),
            app.get_advantage_letter(&app.character.get_history().2),
            app.character.get_history().1,
            app.get_dot(
                app.character.get_insight().0
            ),
            app.get_advantage_letter(&app.character.get_insight().2),
            app.character.get_insight().1,
            app.get_dot(
                app.character.get_intimidation().0
            ),
            app.get_advantage_letter(&app.character.get_intimidation().2),
            app.character.get_intimidation().1,
            app.get_dot(
                app.character.get_investigation().0
            ),
            app.get_advantage_letter(&app.character.get_investigation().2),
            app.character.get_investigation().1,
            app.get_dot(
                app.character.get_medicine().0
            ),
            app.get_advantage_letter(&app.character.get_medicine().2),
            app.character.get_medicine().1,
            app.get_dot(
                app.character.get_nature().0
            ),
            app.get_advantage_letter(&app.character.get_nature().2),
            app.character.get_nature().1,
            app.get_dot(
                app.character.get_perception().0
            ),
            app.get_advantage_letter(&app.character.get_perception().2),
            app.character.get_perception().1,
            app.get_dot(
                app.character.get_performance().0
            ),
            app.get_advantage_letter(&app.character.get_performance().2),
            app.character.get_performance().1,
            app.get_dot(
                app.character.get_persuasion().0
            ),
            app.get_advantage_letter(&app.character.get_persuasion().2),
            app.character.get_persuasion().1,
            app.get_dot(
                app.character.get_religion().0
            ),
            app.get_advantage_letter(&app.character.get_religion().2),
            app.character.get_religion().1,
            app.get_dot(
                app.character.get_sleight_of_hand().0
            ),
            app.get_advantage_letter(&app.character.get_sleight_of_hand().2),
            app.character.get_sleight_of_hand().1,
            app.get_dot(
                app.character.get_stealth().0
            ),
            app.get_advantage_letter(&app.character.get_stealth().2),
            app.character.get_stealth().1,
            app.get_dot(
                app.character.get_survival().0
            ),
            app.get_advantage_letter(&app.character.get_survival().2),
            app.character.get_survival().1,
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
