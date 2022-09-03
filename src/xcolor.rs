use cnx::text::Color;
use colors_transform::{Color as ColorTransform, Rgb};
use xrdb::Colors;

fn read_xresources_color(color_code: usize) -> Color {
    let xcolors = thledbar_colors_from_xresources();
    let hex = xcolors.colors[color_code].as_ref().unwrap();
    let rgb = Rgb::from_hex_str(hex).unwrap();

    Color::from_rgb(
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
    )
}

fn thledbar_colors_from_xresources() -> Colors {
    Colors::new("thledbar").unwrap()
}

pub fn blue() -> Color {
    read_xresources_color(4)
}

pub fn background() -> Color {
    let xcolors = thledbar_colors_from_xresources();
    let hex = xcolors.bg.unwrap();
    let rgb = Rgb::from_hex_str(&hex).unwrap();

    Color::from_rgb(
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
    )
}

pub(crate) fn foreground() -> Color {
    let xcolors = thledbar_colors_from_xresources();
    let hex = xcolors.fg.unwrap();
    let rgb = Rgb::from_hex_str(&hex).unwrap();

    Color::from_rgb(
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
    )
}
