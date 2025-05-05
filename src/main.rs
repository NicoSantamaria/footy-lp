use std::hash::Hash;
use footy_lp::constraints::{
    Game, Team, build_constraints
};

/// TODO: Simplify structs-- only include necessary data
/// TODO: research lifetimes to avoid cloning



fn main() {
    let teams: Vec<Team> = Vec::from([
        Team { name: "Argentina".into(), points: 0 },
        Team { name: "Mexico".into(), points: 1 },
        Team { name: "Poland".into(), points: 4 },
        Team { name: "Saudi Arabia".into(), points: 3 }
    ]);
    let team_refs: Vec<&Team> = teams.iter().collect();

    let games: Vec<Game> = Vec::from([
        Game::new(&teams[0], &teams[1], 0),
        Game::new(&teams[0], &teams[2], 0),
        Game::new(&teams[1], &teams[3], 0),
    ]);


    build_constraints(&teams[0], team_refs, games);
}