use eframe::egui::Context;
use egui::{self, Button, Color32, Ui};

use crate::tournament::{MatchNode, MatchType, Participant, Team, TournamentType};

pub struct TournamentUI {
    pub participants: Vec<Participant>,
    pub match_type: MatchType,
    pub tournament_type: TournamentType,
    pub state: TournamentUIState,
    pub round_number: u8,
    pub matches: Vec<MatchNode>,
    pub winner: Option<Team>,
}

pub enum TournamentUIState {
    PreMatch,
    Ongoing,
    Finished,
}

impl TournamentUI {
    pub fn new() -> Self {
        Self {
            participants: vec![],
            match_type: MatchType::OneVsOne,
            tournament_type: TournamentType::RoundRobin,
            state: TournamentUIState::PreMatch,
            winner: None,
            round_number: 0,
            matches: Vec::new(),
        }
    }

    pub fn render(&mut self, ctx: &eframe::egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| match self.state {
            TournamentUIState::PreMatch => {
                self.render_pre_match(ui);
            }
            TournamentUIState::Ongoing => {
                self.render_ongoing_match(ui, ctx);
            }
            TournamentUIState::Finished => {
                self.render_finished_match(ui);
            }
        });
    }

    fn render_pre_match(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Tournament Bracket Generator");
        });
        ui.separator();
        ui.add_space(10.0);
        // Match type selection
        ui.horizontal(|ui| {
            ui.label("Match Type:");
            ui.radio_value(&mut self.match_type, MatchType::OneVsOne, "1v1");
            ui.radio_value(&mut self.match_type, MatchType::TwoVsTwo, "2v2");
            ui.radio_value(&mut self.match_type, MatchType::ThreeVsThree, "3v3");
            ui.radio_value(&mut self.match_type, MatchType::FourVsFour, "4v4");
        });
        ui.add_space(10.0);

        // Tournament type selection
        ui.horizontal(|ui| {
            ui.label("Tournament Type:");
            ui.radio_value(
                &mut self.tournament_type,
                TournamentType::SingleElimination,
                "Single Elimination",
            )
            .on_hover_text("A Team/Participant is eliminated after one loss.");
            ui.radio_value(
                &mut self.tournament_type,
                TournamentType::DoubleElimination,
                "Double Elimination",
            )
            .on_hover_text("A Team/Participant is eliminated only after two losses.");
            ui.radio_value(
                &mut self.tournament_type,
                TournamentType::RoundRobin,
                "Round Robin",
            )
            .on_hover_text("Each Team/Participant competes against every other participant.");
        });
        ui.colored_label(
            Color32::DARK_GRAY,
            "Hover for more information ont the tournament type.",
        );
        ui.add_space(10.0);

        static mut TEMP_NAME: String = String::new();

        unsafe {
            // Text field for entering participant name
            ui.horizontal(|ui| {
                ui.label("Participant Name: ");
                let response =
                    egui::TextEdit::singleline(&mut TEMP_NAME).hint_text("Enter participant name");
                ui.add(response);
            });

            // Button to add the participant
            if ui.button("Add Participant").clicked() {
                if !TEMP_NAME.trim().is_empty() {
                    self.participants.push(Participant {
                        name: TEMP_NAME.clone(),
                    });
                    TEMP_NAME.clear(); // Reset the temporary input
                }
            }
        }

        for (i, participant) in self.participants.iter().enumerate() {
            ui.label(format!("{}. {}", i + 1, participant.name));
        }

        let check_participants = (self.participants.len() % self.match_type.type_to_number() == 0)
            && self.participants.len() >= 2;

        // Generate bracket button
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            let generate_bracket_button =
                ui.add_enabled(check_participants, Button::new("Generate Bracket"));
            if generate_bracket_button.clicked() {
                self.state = TournamentUIState::Ongoing;
            }
        });
    }

    fn render_ongoing_match(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.round_number = 1;

        crate::tournament::generate_matches(self);

        let teams = &mut self.matches;

        // egui::ScrollArea::both().show(ui, |ui| {
        for (i, team) in teams.iter_mut().enumerate() {
            let x_offset = 20.0;
            let y_offset = 10.0 + (i as f32 * 120.0);

            // Create a window for each match
            egui::Window::new(format!("Match {}", i + 1))
                .title_bar(false)
                .movable(false)
                .collapsible(false)
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(x_offset, y_offset))
                .show(ctx, |ui| {
                    match &team.team1 {
                        Some(team1) => {
                            ui.label(format!("Team {}: {}", i + 1, team1.team_to_string()));

                            // Add DragValue for the score input
                            ui.horizontal(|ui| {
                                ui.label("Score: ");
                                ui.add(
                                    egui::TextEdit::singleline(&mut team.score1.to_string())
                                        .hint_text("Enter score"),
                                );
                            });
                        }
                        None => (),
                    };

                    match &team.team2 {
                        Some(team2) => {
                            ui.separator();

                            ui.label(format!("Team {}: {}", i + 1, team2.team_to_string()));

                            // Add DragValue for the score input
                            ui.horizontal(|ui| {
                                ui.label("Score: ");
                                ui.add(
                                    egui::TextEdit::singleline(&mut team.score1.to_string())
                                        .hint_text("Enter score"),
                                );
                            });
                        }
                        None => (),
                    };
                });
        }
        // });

        egui::TopBottomPanel::bottom("next_round_value").show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let mut button_content = String::new();
                if self.matches.len() == 1 {
                    button_content = "Finish Tournament".to_string();
                } else {
                    button_content = "Next Round".to_string();
                }

                let next_round_button = ui.add_enabled(true, Button::new(button_content));

                if next_round_button.clicked() {
                    crate::tournament::next_round(self);
                    self.round_number += 1;
                }
            });
        });
    }

    fn render_finished_match(&mut self, ui: &mut Ui) {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                ui.heading("Finished Match");
                if let Some(winner) = &self.winner {
                    ui.label("Winner: ");
                    for participant in winner.members.iter() {
                        ui.label(participant.name.clone());
                    }
                } else {
                    ui.label("No winner. Weird?");
                }
            },
        );
    }
}
