use std::hash::Hash;
use std::collections::HashSet;
use footy_lp::constraints::{
    Game, Team, build_constraints
};

/// TODO: Simplify structs-- only include necessary data
/// TODO: research lifetimes to avoid cloning



fn main() {
    let score = 3;
    let teams: Vec<Team> = Vec::from([
        Team { name: "Argentina".into(), points: 0 },
        Team { name: "Mexico".into(), points: 1 },
        Team { name: "Poland".into(), points: 4 },
        Team { name: "Saudi Arabia".into(), points: 3 }
    ]);
    // clean up with references and lifetimes
    let teams_clone = teams.clone();

    let games: Vec<Game> = Vec::from([
        Game::new(teams_clone[0].clone(), teams_clone[1].clone(), 0),
        Game::new(teams_clone[0].clone(), teams_clone[2].clone(), 0),
        Game::new(teams_clone[1].clone(), teams_clone[3].clone(), 0),
    ]);

    let source = teams[0].clone();

    build_constraints(source, score, teams, games);
}

