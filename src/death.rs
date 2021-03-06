use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, RichText},
    EguiContext,
};

use crate::{
    assets::AudioAssets,
    game::{self, HostState},
    menu::button,
    tear_down, GameState, GlobalState,
};

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_exit(GameState::Dead)
                .with_system(tear_down::<game::ScreenTag>)
                .with_system(tear_down::<game::tissue::ScreenTag>),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Dead)
                .with_system(game::ui::status)
                .with_system(death),
        );
    }
}

const TEXT: [&str; 6] = [
    "Already?",
    "Good news! You'll get stronger\nwith each generation.",
    "Seems harder than expected.\nLet's see if you could get...\nan unfair advantage.",
    "Unlocked Progress!\nCheck out what you can get.",
    "Oh, and don't worry about\nthe white cells.\nThat's your immune system\nfinally kicking back!",
    "That was inevitable.",
];

fn death(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    host_state: Res<HostState>,
    mut global_state: ResMut<GlobalState>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let text = if global_state.generation < 5 {
        TEXT[global_state.generation]
    } else {
        TEXT[5]
    };
    egui::Window::new(RichText::new("Death").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.strong(format!("You died at age {:>2.1}.", host_state.age));
                ui.add_space(10.0);
                ui.label(text);
                ui.add_space(30.0);
                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(350.0);
                    button(
                        ui,
                        "Try Again...",
                        || {
                            audio.play(
                                audio_assets.button.clone_weak(),
                                PlaybackSettings {
                                    repeat: false,
                                    speed: 1.0,
                                    volume: 0.2,
                                },
                            );

                            global_state.generation += 1;
                            global_state.progress += host_state.age;
                            global_state.expectancy = host_state.age.max(global_state.expectancy);
                            let _ = state.set(GameState::Menu);
                        },
                        true,
                        false,
                    );
                    ui.add_space(10.0);
                });
            });
        });
}
