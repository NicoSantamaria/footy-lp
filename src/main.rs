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

struct Edge<'a> {
    source_team: HashSet<&'a Team>,
    target_team: HashSet<&'a Team>,
    capacity: Option<u32>,
}

// struct Graph {
//     nodes: Vec<Node>,
//     edges: Vec<Edge>,
// }

// TODO: add this function to its own file, split up into smaller
// functions for easier testing
fn build_constraints<'a>(
    source: &'a Team,
    teams: Vec<&'a Team>,
    remaining_games: Vec<Game<'a>>,
) {
    let possible_games_nodes: Vec<Game<'a>> = teams
        .iter()
        .combinations(2)
        .map(|combo| Game::new(combo[0], combo[1], 0))
        .collect();

    let remaining_points_edges: Vec<RemainingPointsSource> = possible_games_nodes
        .iter()
        .map(|possible_game| {
            let remaining = remaining_games
                .iter()
                .filter(|game| game.teams == possible_game.teams)
                .count() as u32 * 3;
            RemainingPointsSource::new(source, possible_game, remaining)
        })
        .collect();

    let mut infinite_capacity_edges: Vec<Edge> = Vec::new();
    for source in &possible_games_nodes {
        for &target in &teams {
            if source.teams.contains(&target) {
                let mut target_team = HashSet::new();
                target_team.insert(target);

                infinite_capacity_edges.push(Edge {
                    source_team: source.teams.clone(), // Clone the reference set
                    target_team,
                    capacity: None,
                });
            }
        }
    }

    for ice in infinite_capacity_edges {
        println!("{:?}", ice.source_team);
        println!("{:?}", ice.target_team);
        println!("{:?}", ice.capacity);
    }
    // let remaining_distance_edges = vec![];


    // Graph { nodes: vec![], edges: vec![] }
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