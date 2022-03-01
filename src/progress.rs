use std::fmt::{self, Formatter};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, ImageButton, Layout, RichText, Ui, Widget},
    EguiContext,
};

use crate::{assets::ProgressAssets, menu::button, GameState, GlobalState};

pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Progress).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Progress).with_system(progress));
    }
}

enum Progress {
    Disinfectant,
    Antibiotics,
    Vaccine,
    Sanitation,
    PersonalHygiene,
    PreventiveMeasures,
    SickDays,
    FreeHealthcare,
    ParentalLeave,
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Progress::Disinfectant => f.pad("Disinfectant"),
            Progress::Antibiotics => f.pad("Antibiotics"),
            Progress::Vaccine => f.pad("Vaccine"),
            Progress::Sanitation => f.pad("Sanitation"),
            Progress::PersonalHygiene => f.pad("Personal Hygiene"),
            Progress::PreventiveMeasures => f.pad("Preventive Measures"),
            Progress::SickDays => f.pad("Sick Days"),
            Progress::FreeHealthcare => f.pad("Free Healthcare"),
            Progress::ParentalLeave => f.pad("Parental Leave"),
        }
    }
}

impl Progress {
    const fn to_image_id(&self) -> u64 {
        match self {
            Progress::Disinfectant => 0,
            Progress::Antibiotics => 1,
            Progress::Vaccine => 2,
            Progress::Sanitation => 3,
            Progress::PersonalHygiene => 4,
            Progress::PreventiveMeasures => 5,
            Progress::SickDays => 6,
            Progress::FreeHealthcare => 7,
            Progress::ParentalLeave => 8,
        }
    }
}

fn setup(
    mut egui_context: ResMut<EguiContext>,
    assets: Res<ProgressAssets>,
    mut done: Local<bool>,
) {
    if !*done {
        egui_context.set_egui_texture(
            Progress::Disinfectant.to_image_id(),
            assets.disinfectant.clone_weak(),
        );
        egui_context.set_egui_texture(
            Progress::Antibiotics.to_image_id(),
            assets.antibiotics.clone_weak(),
        );
        egui_context.set_egui_texture(Progress::Vaccine.to_image_id(), assets.vaccine.clone_weak());
        egui_context.set_egui_texture(
            Progress::Sanitation.to_image_id(),
            assets.sanitation.clone_weak(),
        );
        egui_context.set_egui_texture(
            Progress::PersonalHygiene.to_image_id(),
            assets.personal_hygiene.clone_weak(),
        );
        egui_context.set_egui_texture(
            Progress::PreventiveMeasures.to_image_id(),
            assets.preventive_measures.clone_weak(),
        );
        egui_context.set_egui_texture(
            Progress::SickDays.to_image_id(),
            assets.sick_days.clone_weak(),
        );
        egui_context.set_egui_texture(
            Progress::FreeHealthcare.to_image_id(),
            assets.free_healthcare.clone_weak(),
        );
        egui_context.set_egui_texture(
            Progress::ParentalLeave.to_image_id(),
            assets.parental_leave.clone_weak(),
        );

        *done = true;
    }
}

impl Progress {
    fn base_and_factor(&self) -> (usize, usize) {
        match self {
            Progress::Disinfectant => (25, 1),
            Progress::Antibiotics => (25, 2),
            Progress::Vaccine => (25, 5),
            Progress::PersonalHygiene => (50, 5),
            Progress::Sanitation => (50, 10),
            Progress::PreventiveMeasures => (50, 25),
            Progress::SickDays => (100, 25),
            Progress::FreeHealthcare => (100, 50),
            Progress::ParentalLeave => (100, 100),
        }
    }
}

impl GlobalState {
    fn has(&self, progress: &Progress) -> bool {
        match progress {
            Progress::Disinfectant => self.disinfectant != usize::MAX,
            Progress::Antibiotics => self.antibiotics != usize::MAX,
            Progress::Vaccine => self.vaccine != usize::MAX,
            Progress::Sanitation => self.sanitation != usize::MAX,
            Progress::PersonalHygiene => self.personal_hygiene != usize::MAX,
            Progress::PreventiveMeasures => self.preventive_measures != usize::MAX,
            Progress::SickDays => self.sick_days != usize::MAX,
            Progress::FreeHealthcare => self.free_healthcare != usize::MAX,
            Progress::ParentalLeave => self.parental_leave != usize::MAX,
        }
    }

    fn get(&mut self, progress: &Progress) {
        match progress {
            Progress::Disinfectant => self.disinfectant = self.generation,
            Progress::Antibiotics => self.antibiotics = self.generation,
            Progress::Vaccine => self.vaccine = self.generation,
            Progress::Sanitation => self.sanitation = self.generation,
            Progress::PersonalHygiene => self.personal_hygiene = self.generation,
            Progress::PreventiveMeasures => self.preventive_measures = self.generation,
            Progress::SickDays => self.sick_days = self.generation,
            Progress::FreeHealthcare => self.free_healthcare = self.generation,
            Progress::ParentalLeave => self.parental_leave = self.generation,
        }
    }

    fn cost(&self, progress: &Progress) -> usize {
        progress.base_and_factor().0
            + progress.base_and_factor().1 * (self.current_progress_multiplier() + 2)
    }
}

fn progress(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut global_state: ResMut<GlobalState>,
) {
    egui::Window::new(RichText::new("Cleanse").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.strong(format!("Progress Points: {:.0}", global_state.progress));
                ui.separator();

                ui.horizontal(|ui| {
                    image_button(ui, Progress::Disinfectant, &mut *global_state);
                    image_button(ui, Progress::Antibiotics, &mut *global_state);
                    image_button(ui, Progress::Vaccine, &mut *global_state);
                });

                ui.horizontal(|ui| {
                    image_button(ui, Progress::PersonalHygiene, &mut *global_state);
                    image_button(ui, Progress::Sanitation, &mut *global_state);
                    image_button(ui, Progress::PreventiveMeasures, &mut *global_state);
                });

                ui.horizontal(|ui| {
                    image_button(ui, Progress::SickDays, &mut *global_state);
                    image_button(ui, Progress::FreeHealthcare, &mut *global_state);
                    image_button(ui, Progress::ParentalLeave, &mut *global_state);
                });

                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    ui.set_width(350.);
                    button(
                        ui,
                        "Back",
                        || {
                            let _ = state.set(GameState::Menu);
                        },
                        true,
                        false,
                    );
                });
                ui.add_space(10.0);
            });
        });
}

fn image_button(ui: &mut Ui, progress: Progress, global_state: &mut GlobalState) {
    ui.with_layout(Layout::left_to_right(), |ui| {
        ui.set_width(300.0);

        let cost = global_state.cost(&progress);

        if match (global_state.has(&progress), cost) {
            (true, _) => true,
            (_, x) if (x as f32) > global_state.progress => true,
            _ => false,
        } {
            ui.set_enabled(false);
        }

        if ImageButton::new(
            egui::TextureId::User(progress.to_image_id()),
            egui::vec2(48.0, 48.0),
        )
        .tint(match (global_state.has(&progress), cost) {
            (true, _) => Color32::DARK_GREEN,
            (_, x) if (x as f32) > global_state.progress => Color32::DARK_GRAY,
            _ => Color32::WHITE,
        })
        .ui(ui)
        .clicked()
        {
            global_state.progress -= cost as f32;
            global_state.get(&progress);
        }
        ui.small(&format!("{:<20}\n\n{}", progress, cost));
    });
}
