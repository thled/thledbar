use cnx::text::Color;
use xrdb::Colors;

pub fn background() -> Color {
    let hex = xcolors().bg.unwrap();

    Color::from_hex(&hex)
}

pub fn foreground() -> Color {
    let hex = xcolors().fg.unwrap();

    Color::from_hex(&hex)
}

pub fn red() -> Color {
    read_xresources_color(1)
}

pub fn blue() -> Color {
    read_xresources_color(4)
}

pub fn yellow() -> Color {
    read_xresources_color(3)
}

fn xcolors() -> Colors {
    Colors::new("thledbar").unwrap()
}

fn read_xresources_color(color_code: usize) -> Color {
    let xcolor = xcolors();
    let hex = xcolor.colors[color_code].as_ref().unwrap();

    Color::from_hex(hex)
}
