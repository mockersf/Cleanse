#![windows_subsystem = "windows"]

#[cfg(feature = "hot")]
use bevy::asset::AssetServerSettings;
use bevy::{app::AppExit, audio::AudioSink, prelude::*};
#[cfg(not(feature = "release"))]
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::{Level, LogSettings},
    window::PresentMode,
};

use bevy_easings::EasingsPlugin;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::physics::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

mod assets;
mod bloodfield;
mod camera;
mod cheat;
mod death;
mod game;
pub mod menu;
mod progress;
mod splash;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Cleanse".to_string(),
        #[cfg(not(feature = "release"))]
        present_mode: PresentMode::Immediate,
        resizable: false,
        decorations: false,
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., 0.).into(),
            ..Default::default()
        })
        .add_plugin(EasingsPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Exit).with_system(exit))
        .add_state(GameState::Splash)
        .add_plugin(assets::AssetPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(bloodfield::BloodfieldPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(cheat::CheatPlugin)
        .add_plugin(progress::ProgressPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(death::DeathPlugin)
        .insert_resource(GlobalState {
            generation: 0,
            expectancy: 0.0,
            progress: 0.0,
            disinfectant: usize::MAX,
            antibiotics: usize::MAX,
            vaccine: usize::MAX,
            personal_hygiene: usize::MAX,
            sanitation: usize::MAX,
            preventive_measures: usize::MAX,
            sick_days: usize::MAX,
            free_healthcare: usize::MAX,
            parental_leave: usize::MAX,
        })
        .insert_resource(UxState {
            background_loop: None,
        })
        .run();
}

pub struct UxState {
    pub background_loop: Option<Handle<AudioSink>>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Cheat,
    Progress,
    Playing,
    LevelUp,
    Dead,
    Oldest,
    Exit,
    Intro,
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

pub struct GlobalState {
    pub generation: usize,
    pub expectancy: f32,
    pub progress: f32,
    pub disinfectant: usize,
    pub antibiotics: usize,
    pub vaccine: usize,
    pub personal_hygiene: usize,
    pub sanitation: usize,
    pub preventive_measures: usize,
    pub sick_days: usize,
    pub free_healthcare: usize,
    pub parental_leave: usize,
}

impl GlobalState {
    fn current_progress_multiplier(&self) -> usize {
        (if self.disinfectant != usize::MAX {
            1
        } else {
            0
        }) + (if self.antibiotics != usize::MAX { 1 } else { 0 })
            + (if self.vaccine != usize::MAX { 1 } else { 0 })
            + (if self.personal_hygiene != usize::MAX {
                1
            } else {
                0
            })
            + (if self.sanitation != usize::MAX { 1 } else { 0 })
            + (if self.preventive_measures != usize::MAX {
                1
            } else {
                0
            })
            + (if self.sick_days != usize::MAX { 1 } else { 0 })
            + (if self.free_healthcare != usize::MAX {
                1
            } else {
                0
            })
            + (if self.parental_leave != usize::MAX {
                1
            } else {
                0
            })
    }
}
