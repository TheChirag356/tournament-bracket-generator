use eframe::egui::Context;
use egui::{self, Button, Color32, Ui};

use crate::tournament::{MatchNode, MatchType, Participant, Team, TournamentType};

pub struct TournamentUI {
    pub participants: Vec<Participant>,
    pub match_type: MatchType,
    pub tournament_type: TournamentType,
    pub state: TournamentUIState,
    pub winner: Option<Team>,
    pub round_number: u8,
    pub matches: Vec<MatchNode>,
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
                // self.render_ongoing_match(ui, ctx);
                self.render_tournament_graph(ui, ctx);
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
        // Match type selection
        ui.horizontal(|ui| {
            ui.label("Match Type:");
            ui.radio_value(&mut self.match_type, MatchType::OneVsOne, "1v1");
            ui.radio_value(&mut self.match_type, MatchType::TwoVsTwo, "2v2");
            ui.radio_value(&mut self.match_type, MatchType::ThreeVsThree, "3v3");
            ui.radio_value(&mut self.match_type, MatchType::FourVsFour, "4v4");
        });

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

    fn render_ongoing_match(&mut self, ui: &mut egui::Ui, ctx: &eframe::egui::Context) {
        // Increment round number if this function is tied to advancing the rounds
        self.round_number += 1;

        // Create teams for the match
        let teams = crate::tournament::create_teams(&self.participants, self.match_type.clone());

        for team in teams.iter() {
            ui.label(team.team_to_string());
        }
    }

    fn render_tournament_graph(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let painter = ui.painter(); // Get a painter for custom rendering
        crate::tournament::generate_matches(self);

        let mut match_positions = Vec::new(); // Store positions of match nodes

        let node_width = 150.0;
        let node_height = 50.0;
        let padding = 20.0;

        let mut x_offset = 10.0;
        let mut y_offset = 10.0;

        for (i, match_node) in self.matches.iter().enumerate() {
            // Calculate node position
            let pos = egui::pos2(x_offset, y_offset);

            // Draw the match node as a rectangle
            let rect = egui::Rect::from_min_size(pos, egui::vec2(node_width, node_height));
            painter.rect_filled(rect, 5.0, egui::Color32::from_gray(100));

            // Add team names and scores
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!(
                    "{} ({})\n{} ({})",
                    match_node.team1, match_node.score1, match_node.team2, match_node.score2
                ),
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );

            // Store position for edge drawing
            match_positions.push(rect);

            // Adjust offsets for the next node
            y_offset += node_height + padding;

            if (i + 1) % 4 == 0 {
                // Example: Move to the next column every 4 nodes
                x_offset += node_width + padding;
                y_offset = 0.0;
            }
        }

        // Draw edges between match nodes
        for (i, match_node) in self.matches.iter().enumerate() {
            if let Some(next_index) = match_node.next_match {
                if let Some(start_rect) = match_positions.get(i) {
                    if let Some(end_rect) = match_positions.get(next_index) {
                        let start = start_rect.center_bottom(); // Start at the bottom of the current match
                        let end = end_rect.center_top(); // End at the top of the next match
                        painter.line_segment(
                            [start, end],
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                        );
                    }
                }
            }
        }
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
