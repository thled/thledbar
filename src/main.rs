use std::error::Error;
use std::time::Duration;

use cnx::text::*;
use cnx::widgets::*;
use cnx::{Cnx, Position};
use cnx_contrib::widgets::battery::*;
use cnx_contrib::widgets::command::Command;
use cnx_contrib::widgets::cpu;

mod xcolor;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cnx = Cnx::new(Position::Top);

    cnx.add_widget(workspaces());
    cnx.add_widget(window_title());
    cnx.add_widget(pomodoro());
    cnx.add_widget(cpu()?);
    cnx.add_widget(battery());
    cnx.add_widget(clock());

    cnx.run()?;

    Ok(())
}

fn workspaces() -> Pager {
    let active_attr = Attributes {
        fg_color: xcolor::background(),
        bg_color: Some(xcolor::blue()),
        ..default_attr()
    };
    let inactive_attr = Attributes {
        fg_color: xcolor::blue(),
        ..default_attr()
    };
    let pager_attrs = PagerAttributes {
        active_attr,
        inactive_attr,
        non_empty_attr: default_attr(),
    };

    Pager::new(pager_attrs)
}

fn window_title() -> ActiveWindowTitle {
    ActiveWindowTitle::new(default_attr())
}

fn pomodoro() -> Command {
    Command::new(
        default_attr(),
        "uairctl fetch '{time}' | cut -d' ' -f1".into(),
        Duration::from_secs(10),
    )
}

fn cpu() -> Result<cpu::Cpu, anyhow::Error> {
    let render = Box::new(|load| {
        let mut color = xcolor::foreground().to_hex();
        if load > 25 {
            color = xcolor::yellow().to_hex();
        }
        if load > 75 {
            color = xcolor::red().to_hex();
        }
        format!("CPU <span foreground=\"{}\">{}%</span>", color, load)
    });
    cpu::Cpu::new(default_attr(), Some(render))
}

fn battery() -> Battery {
    Battery::new(default_attr(), Color::red(), None, None)
}

fn clock() -> Clock {
    Clock::new(default_attr(), Some("%Y-%m-%d %a %I:%M %p".into()))
}

fn default_attr() -> Attributes {
    Attributes {
        font: Font::new("Sauce Code Pro Nerd Font Semibold 12"),
        fg_color: xcolor::foreground(),
        bg_color: Some(xcolor::background()),
        padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    }
}
