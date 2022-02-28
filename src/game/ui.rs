use bevy::prelude::*;
use bevy_egui::{
    egui::{self, text::LayoutJob, Color32, TextFormat, WidgetText},
    EguiContext,
};

use super::{immune_system::ImmuneSystem, HostState};

pub fn status(
    mut egui_context: ResMut<EguiContext>,
    state: Res<HostState>,
    immune_system: Query<&ImmuneSystem>,
) {
    egui::TopBottomPanel::top("top").show(egui_context.ctx_mut(), |ui| {
        let mut status_to_display = LayoutJob::default();

        status_to_display.append(
            &format!(" {:10} ", state.status),
            0.0,
            match state.status {
                super::Status::Healthy => TextFormat {
                    color: Color32::WHITE,
                    background: Color32::GREEN,
                    ..Default::default()
                },
                super::Status::Sick => TextFormat {
                    color: Color32::WHITE,
                    background: Color32::RED,
                    ..Default::default()
                },
                super::Status::Dead => TextFormat {
                    color: Color32::BLACK,
                    background: Color32::RED,
                    ..Default::default()
                },
            },
        );

        status_to_display.append(
            " ",
            0.0,
            TextFormat::simple(egui::TextStyle::Body, Color32::GRAY),
        );

        let immune_system = immune_system.single();
        status_to_display.append(
            &"-".repeat((immune_system.health / immune_system.original_health * 50.0) as usize),
            0.0,
            TextFormat {
                color: Color32::GREEN,
                background: Color32::GREEN,
                ..Default::default()
            },
        );
        status_to_display.append(
            &"-".repeat(
                ((1.0 - immune_system.health / immune_system.original_health) * 50.0) as usize,
            ),
            0.0,
            TextFormat {
                color: Color32::RED,
                background: Color32::RED,
                ..Default::default()
            },
        );

        status_to_display.append(
            "   -   ",
            0.0,
            TextFormat::simple(egui::TextStyle::Body, Color32::GRAY),
        );

        status_to_display.append(
            "age",
            0.0,
            TextFormat::simple(egui::TextStyle::Body, Color32::WHITE),
        );
        status_to_display.append(
            &format!("  {:>2.1}", state.age),
            0.0,
            TextFormat::simple(egui::TextStyle::Body, Color32::WHITE),
        );

        ui.label(WidgetText::LayoutJob(status_to_display));
    });
}
