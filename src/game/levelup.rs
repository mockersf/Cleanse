use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, ImageButton, Layout, RichText, Ui, Widget},
    EguiContext,
};

use crate::{progress::Progress, GameState};

use super::immune_system::ImmuneSystem;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_update(GameState::LevelUp).with_system(levelup));
    }
}

fn levelup(
    mut egui_context: ResMut<EguiContext>,
    mut immune_system: Query<&mut ImmuneSystem>,
    mut state: ResMut<State<GameState>>,
) {
    egui::Window::new(RichText::new("Level Up!").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.separator();

                ui.horizontal(|ui| {
                    let mut immune_system = immune_system.single_mut();

                    image_button(ui, Progress::LevelUpAttack, || {
                        immune_system.attack_spawn_rate += 0.2;
                        let _ = state.pop();
                    });
                    image_button(ui, Progress::LevelUpSpeed, || {
                        immune_system.speed += 15.0;
                        let _ = state.pop();
                    });
                    image_button(ui, Progress::LevelUpHealth, || {
                        immune_system.original_health += 25.0;
                        immune_system.health = immune_system.original_health;
                        let _ = state.pop();
                    });
                });

                ui.add_space(10.0);
            });
        });
}

fn image_button(ui: &mut Ui, progress: Progress, mut on_click: impl FnMut()) {
    ui.with_layout(Layout::left_to_right(), |ui| {
        ui.set_width(300.0);

        if ImageButton::new(
            egui::TextureId::User(progress.to_image_id()),
            egui::vec2(48.0, 48.0),
        )
        .tint(Color32::WHITE)
        .ui(ui)
        .clicked()
        {
            on_click()
        }
        ui.small(&format!("{}", progress));
    });
}
