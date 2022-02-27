#![windows_subsystem = "windows"]

#[cfg(feature = "hot")]
use bevy::asset::AssetServerSettings;
use bevy::{app::AppExit, prelude::*};
#[cfg(not(feature = "release"))]
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogSettings},
    window::PresentMode,
};

use bevy_egui::EguiPlugin;

mod assets;
mod bloodfield;
mod camera;
mod game;
mod menu;
mod splash;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Cleanse".to_string(),
        #[cfg(not(feature = "release"))]
        present_mode: PresentMode::Immediate,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::BLACK));

    #[cfg(feature = "release")]
    app.insert_resource(bevy::log::LogSettings {
        level: bevy::log::Level::WARN,
        ..Default::default()
    });

    #[cfg(feature = "hot")]
    app.insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..Default::default()
    });

    #[cfg(not(feature = "release"))]
    app.insert_resource(LogSettings {
        level: Level::TRACE,
        filter: "wgpu=warn,bevy=info,winit=info,naga=info".to_string(),
    });
    app.add_plugins_with(DefaultPlugins, |group| {
        #[cfg(feature = "bundled")]
        group.add_before::<bevy::asset::AssetPlugin, _>(bevy_embedded_assets::EmbeddedAssetPlugin);
        group
    });
    #[cfg(not(feature = "release"))]
    {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::filtered(vec![
                FrameTimeDiagnosticsPlugin::FPS,
            ]));
    }
    app.add_plugin(EguiPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Exit).with_system(exit))
        .add_state(GameState::Splash)
        .add_plugin(assets::AssetPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(bloodfield::BloodfieldPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Playing,
    Exit,
}

pub fn tear_down<Tag: Component>(mut commands: Commands, query: Query<Entity, With<Tag>>) {
    debug!("Tear Down {:?}", std::any::type_name::<Tag>());

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
