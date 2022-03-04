use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, RichText},
    EguiContext,
};

use crate::{assets::AudioAssets, menu::button, GameState, GlobalState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_update(GameState::Intro).with_system(maybe_skip))
            .add_system_set(SystemSet::on_update(GameState::Intro).with_system(intro));
    }
}

fn maybe_skip(mut state: ResMut<State<GameState>>, global_state: Res<GlobalState>) {
    if global_state.generation > 4 {
        let _ = state.pop();
    }
}

const TEXTS: [[&str; 3]; 5] = [
    ["Welcome", "Just avoid contamination.", "Sounds easy!"],
    [
        "Welcome back!",
        "Stay in the blood vessel!\nYou're stronger there.",
        "That should help.",
    ],
    [
        "Should have mentionned...",
        "The blood vessel is the red\nzone at the center.",
        "Oh, OK.",
    ],
    [
        "A normal life expectancy?",
        "That's like, 300 years old.\nTotally doable, just dodge those pathogens.",
        "I'm... almost there?",
    ],
    ["Well", "You're on your own now!", "I got all I need."],
];

fn intro(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    global_state: Res<GlobalState>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let (title, body, valid) = if global_state.generation < 5 {
        (
            TEXTS[global_state.generation][0],
            TEXTS[global_state.generation][1],
            TEXTS[global_state.generation][2],
        )
    } else {
        ("", "", "")
    };
    egui::Window::new(RichText::new(title).color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.label(body);

                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(350.0);
                    ui.add_space(20.0);
                    button(
                        ui,
                        valid,
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
