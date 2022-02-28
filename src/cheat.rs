use bevy::prelude::*;
use bevy_egui::{
    egui::{self, text::LayoutJob, Align2, Color32, RichText, TextFormat},
    EguiContext,
};

use crate::{assets::LoadingState, menu::button, GameState, GlobalState};

pub struct CheatPlugin;

impl Plugin for CheatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_update(GameState::Cheat).with_system(cheat));
    }
}

fn cheat(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    asset_state: Res<State<LoadingState>>,
    mut global_state: ResMut<GlobalState>,
) {
    egui::Window::new(RichText::new("Cleanse").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(350.0);
                    let mut generation = LayoutJob::default();
                    generation.append(
                        "Increase generation",
                        0.0,
                        TextFormat::simple(egui::TextStyle::Button, Color32::WHITE),
                    );
                    generation.append(
                        &format!("\ngeneration {}", global_state.generation),
                        0.0,
                        TextFormat::simple(egui::TextStyle::Small, Color32::GRAY),
                    );

                    button(
                        ui,
                        generation,
                        || {
                            global_state.generation += 1;
                        },
                        asset_state.current() != &LoadingState::Assets,
                    );
                    ui.add_space(20.0);
                    // extra space so that back button is not aligned with other in menu
                    ui.add_space(40.0);
                    button(
                        ui,
                        "Back",
                        || {
                            let _ = state.set(GameState::Menu);
                        },
                        true,
                    );
                    ui.add_space(10.0);
                });
            });
        });
}
