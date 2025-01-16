use crate::ui::TournamentUI;

#[derive(Clone)]
pub struct Participant {
    pub name: String,
}

#[derive(Clone)]
pub struct Team {
    pub members: Vec<Participant>,
}

#[derive(Clone)]
pub struct MatchNode {
    pub team1: Option<Team>,
    pub team2: Option<Team>,
    pub score1: i32,
    pub score2: i32,
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

impl MatchNode {
    pub fn is_winner(&self) -> Team {
        if self.score1 > self.score2 {
            self.team1.clone().unwrap()
        } else {
            self.team2.clone().unwrap()
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
                team1: Some(team[0].clone()),
                team2: Some(team[1].clone()),
                score1: 0,
                score2: 0,
            });
        }
    }
}

// matches: &Vec<MatchNode>, tournament_type: TournamentType
pub fn next_round(tournamentui: &mut TournamentUI) {
    if tournamentui.matches.len() == 1 {
        if tournamentui.matches[0].score1 > tournamentui.matches[0].score2 {
            tournamentui.winner = tournamentui.matches[0].team1.clone();
        } else if tournamentui.matches[0].score1 < tournamentui.matches[0].score2 {
            tournamentui.winner = tournamentui.matches[0].team2.clone();
        } else {
            tournamentui.winner = None;
        }
    }

    let mut teams: Vec<Team> = Vec::new();
    // Create new matches from winners
    for matches in tournamentui.matches.iter() {
        teams.push(matches.is_winner());
    }

    tournamentui.matches.clear();
    for (i, team) in teams.chunks(2).enumerate() {
        if team.len() == 2 {
            tournamentui.matches.push(MatchNode {
                team1: Some(team[0].clone()),
                team2: Some(team[1].clone()),
                score1: 0,
                score2: 0,
            });
        }
    }
}
