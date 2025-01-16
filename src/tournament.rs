use crate::ui::TournamentUI;

#[derive(Clone)]
pub struct Participant {
    pub name: String,
}

#[derive(Clone)]
pub struct Team {
    pub members: Vec<Participant>,
}

#[derive(Debug, Clone)]
pub struct MatchNode {
    pub team1: String,
    pub team2: String,
    pub score1: i32,
    pub score2: i32,
    pub next_match: Option<usize>, // Index of the next match node
}

#[derive(Debug)]
pub struct TournamentGraph {
    pub matches: Vec<MatchNode>, // All match nodes
}

#[derive(PartialEq, Clone)]
pub enum TournamentType {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
}

#[derive(PartialEq, Clone)]
pub enum MatchType {
    OneVsOne,
    TwoVsTwo,
    ThreeVsThree,
    FourVsFour,
}

impl MatchType {
    pub fn type_to_number(&self) -> usize {
        match self {
            MatchType::OneVsOne => 1,
            MatchType::TwoVsTwo => 2,
            MatchType::ThreeVsThree => 3,
            MatchType::FourVsFour => 4,
        }
    }
}

impl Team {
    pub fn team_to_string(&self) -> String {
        let mut team_string = String::new();
        let members = self.members.clone();

        for (index, member) in members.iter().enumerate() {
            if index == 0 {
                team_string.push_str(&member.name.as_str());
            } else if index > 0 && index < members.len() - 1 {
                team_string.push_str(&format!(", {}", &member.name.as_str()));
            } else if index == members.len() - 1 {
                team_string.push_str(&format!(" & {}", &member.name.as_str()));
            }
        }

        team_string
    }
}

// Co-Pilot Code for create teams
// pub fn create_teams(participants: Vec<Participant>, match_type: MatchType) -> Vec<Team> {
//     let team_size = match match_type {
//         MatchType::OneVsOne => 1,
//         MatchType::TwoVsTwo => 2,
//         MatchType::ThreeVsThree => 3,
//         MatchType::FourVsFour => 4,
//     };
//     participants
//         .chunks(team_size)
//         .map(|chunk| Team {
//             members: chunk.to_vec(),
//         })
//         .collect()
// }

pub fn create_teams(participants: &Vec<Participant>, match_type: MatchType) -> Vec<Team> {
    let team_size = match match_type {
        MatchType::OneVsOne => 1,
        MatchType::TwoVsTwo => 2,
        MatchType::ThreeVsThree => 3,
        MatchType::FourVsFour => 4,
    };

    let mut teams = Vec::new();
    for chunk in participants.chunks(team_size) {
        let mut members = Vec::new();
        for participant in chunk {
            members.push(participant.clone());
        }
        teams.push(Team { members });
    }
    teams
}

pub fn generate_matches(tournamentui: &mut TournamentUI) {
    tournamentui.matches.clear(); // Clear previous matches

    // Example: Create matches from participants
    let teams = create_teams(&tournamentui.participants, tournamentui.match_type.clone());

    for (i, team) in teams.chunks(2).enumerate() {
        if team.len() == 2 {
            tournamentui.matches.push(MatchNode {
                team1: team[0].team_to_string(),
                team2: team[1].team_to_string(),
                score1: 0,
                score2: 0,
                next_match: None, // You can calculate and assign this later
            });
        }
    }
}

pub fn generate_bracket(teams: &Vec<Team>, tournament_type: TournamentType) -> Vec<MatchNode> {
    todo!();
}
