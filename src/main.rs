use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;

/// TODO: Simplify structs-- only include necessary data
/// TODO: research lifetimes to avoid cloning

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Team {
    name: String,
    points: u32,
}

struct RemainingPointsSource<'a> {
    source_team: &'a Team,
    target_game: &'a Game<'a>,
    remaining: u32,
}

impl<'a> RemainingPointsSource<'a> {
    fn new(
        source_team: &'a Team,
        target_game: &'a Game,
        remaining: u32
    ) -> Self {
        Self { source_team, target_game, remaining }
    }
}

#[derive(Debug)]
struct Game<'a> {
    teams: HashSet<&'a Team>,
    number: u32,
}

impl<'a> Game<'a> {
    fn new(
        team1: &'a Team,
        team2: &'a Team, number: u32
    ) -> Self {
        let teams: HashSet<&'a Team> = HashSet::from([team1, team2]);
        Self { teams, number }
    }
}

struct Node {
    teams: HashSet<Team>,
    capacity: u32
}

struct Edge {
    source_team: Team,
    target_team: Team,
    capacity: u32
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn build_constraints<'a>(
    team: &'a Team,
    opponents: Vec<&'a Team>,
    remaining_games: Vec<Game<'a>>,
) {
    let possible_games: Vec<Game<'a>> = opponents
        .iter()
        .combinations(2)
        .map(|combo| Game::new(combo[0], combo[1], 0))
        .collect();

    let remaining_points_sources: Vec<RemainingPointsSource> = possible_games
        .iter()
        .map(|possible_game| {
            let remaining = remaining_games
                .iter()
                .filter(|game| game.teams == possible_game.teams)
                .count() as u32 * 3;
            RemainingPointsSource::new(team, possible_game, remaining)
        })
        .collect();
}

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