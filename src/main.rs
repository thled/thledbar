use std::error::Error;

use cnx::text::*;
use cnx::widgets::*;
use cnx::{Cnx, Position};

fn main() -> Result<(), Box<dyn Error>> {
    let attr = Attributes {
        font: Font::new("Sauce Code Pro Nerd Font Semibold 12"),
        fg_color: Color::white(),
        bg_color: None,
        padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    };

    let mut cnx = Cnx::new(Position::Top);

    cnx.add_widget(ActiveWindowTitle::new(attr.clone()));

    let time_template = Some("%Y-%m-%d %a %I:%M %p".into());
    cnx.add_widget(Clock::new(attr.clone(), time_template));

    cnx.run()?;

    Ok(())
}
