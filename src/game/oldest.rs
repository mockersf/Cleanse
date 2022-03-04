use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, RichText},
    EguiContext,
};

use crate::{assets::AudioAssets, game, menu::button, GameState};

pub struct OldestPlugin;

impl Plugin for OldestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_exit(GameState::Oldest))
            .add_system_set(
                SystemSet::on_update(GameState::Oldest)
                    .with_system(game::ui::status)
                    .with_system(oldest),
            );
    }
}

fn oldest(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    egui::Window::new(RichText::new("Congratulation!").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.strong("You're now the oldest person ever.");
                ui.add_space(20.0);
                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(500.0);
                    button(
                        ui,
                        "My heart will go on...",
                        || {
                            audio.play(
                                audio_assets.button.clone_weak(),
                                PlaybackSettings {
                                    repeat: false,
                                    speed: 1.0,
                                    volume: 0.2,
                                },
                            );
                            let _ = state.pop();
                        },
                        true,
                        false,
                    );
                    ui.add_space(10.0);
                });
            });
        });
}
