use bevy::prelude::*;
use bevy_egui::{
    egui::{self, text::LayoutJob, Color32, TextFormat, WidgetText},
    EguiContext,
};

use super::HostState;

pub fn status(mut egui_context: ResMut<EguiContext>, state: Res<HostState>) {
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
