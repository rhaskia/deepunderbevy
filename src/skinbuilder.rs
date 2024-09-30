use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

pub async fn build_skin() -> Skin {
    let font = load_ttf_font("./fonts/JetBrainsMonoNerdFont.ttf")
            .await
            .unwrap();

    let label_style = root_ui()
        .style_builder()
        .with_font(&font)
        .unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .build();

    Skin {
        label_style,
        ..root_ui().default_skin()
    }
}
