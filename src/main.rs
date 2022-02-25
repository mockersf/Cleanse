use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world)
        .run()
}

fn hello_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Demonstrate changing translation
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Cleanse", text_style.clone(), text_alignment),
        ..Default::default()
    });
}
