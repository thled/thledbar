use std::error::Error;

use cnx::text::*;
use cnx::widgets::*;
use cnx::{Cnx, Position};
use cnx_contrib::widgets::cpu;

fn main() -> Result<(), Box<dyn Error>> {
    let attr = Attributes {
        font: Font::new("Sauce Code Pro Nerd Font Semibold 12"),
        fg_color: Color::white(),
        bg_color: None,
        padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    };

    let mut cnx = Cnx::new(Position::Top);

    let mut active_attr = attr.clone();
    active_attr.bg_color = Some(Color::blue());
    cnx.add_widget(Pager::new(active_attr, attr.clone()));

    cnx.add_widget(ActiveWindowTitle::new(attr.clone()));

    let render = Box::new(|load| {
        let mut color = Color::white().to_hex();
        if load > 10 {
            color = Color::yellow().to_hex();
        }
        if load > 50 {
            color = Color::red().to_hex();
        }
        format!(
            "CPU <span foreground=\"{}\">{}%</span>",
            color,
            load,
        )
    });
    let cpu = cpu::Cpu::new(attr.clone(), Some(render))?;
    cnx.add_widget(cpu);

    let time_template = Some("%Y-%m-%d %a %I:%M %p".into());
    cnx.add_widget(Clock::new(attr.clone(), time_template));

    cnx.run()?;

    Ok(())
}
