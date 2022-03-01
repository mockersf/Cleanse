use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, RichText},
    EguiContext,
};

use crate::{
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

const TEXT: [&str; 5] = [
    "Already?",
    "Good news! You'll get stronger\nwith each generation.",
    "Seems harder than expected.\nLet's see if you can get an...\nunfair advantage.",
    "Unlocked Progress!\nCheck out what you can get.",
    "That was inevitable.",
];

fn death(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    host_state: Res<HostState>,
    mut global_state: ResMut<GlobalState>,
) {
    let text = if global_state.generation < 4 {
        TEXT[global_state.generation]
    } else {
        TEXT[4]
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
