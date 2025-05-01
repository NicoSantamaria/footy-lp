use std::collections::HashSet;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Team {
    name: String,
    points: u32,
}

struct RemainingPoints {
    source_team: Team,
    target_team: Team,
    remaining: u32,
}

struct Game {
    teams: HashSet<Team>,
    number: u32
}

impl Game {
    fn new(team1: &Team, team2: &Team, number: u32) -> Game {
        let mut teams: HashSet<Team> = HashSet::new();
        teams.insert(team1.clone());
        teams.insert(team2.clone());
        Game { teams, number }
    }
}

pub fn build_constraints(
    // team: &Team,
    opponents: Vec<Team>,
    // games: Vec<Game>
) {

    let possible_games: Vec<Game> = opponents
        .iter().combinations(2)
        .map(|combo: Vec<&Team>| Game::new(combo[0], combo[1], 0))
        .collect();

    for game in possible_games {
        println!("{:?}", game.teams);
    }
}

fn main() {
    let teams: Vec<Team> = Vec::from([
        Team { name: "Argentina".into(), points: 0 },
        Team { name: "Mexico".into(), points: 1 },
        Team { name: "Poland".into(), points: 4 },
        Team { name: "Saudi Arabia".into(), points: 3 }
    ]);

    let games: Vec<Game> = Vec::from([
        Game::new(&teams[0], &teams[1], 0),
        Game::new(&teams[0], &teams[2], 0),
        Game::new(&teams[1], &teams[3], 0),
    ]);

    build_constraints(teams);
    // build_constraints(&teams[0], teams, games);
}