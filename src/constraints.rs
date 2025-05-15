use std::collections::HashSet;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Team {
    pub name: String,
    pub points: u32,
}

#[derive(Debug, Clone)]
pub struct Game<'a> {
    pub teams: HashSet<&'a Team>,
    pub number: u32,
}

impl<'a> Game<'a> {
    pub fn new(
        team1: &'a Team,
        team2: &'a Team, number: u32
    ) -> Self {
        let teams: HashSet<&'a Team> = HashSet::from([team1, team2]);
        Self { teams, number }
    }
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

struct Node {
    teams: HashSet<Team>
}

// should probably indicate whether the edge has integer constraints
struct Edge<'a> {
    source_team: HashSet<&'a Team>,
    target_team: HashSet<&'a Team>,
    capacity: Option<u32>,
}

// struct Graph {
//     nodes: Vec<Node>,
//     edges: Vec<Edge>,
// }

// Needs to return Vec<Node>
// OR: call separate function to get nodes, since following functions needs Vec<Game>
// OR: return both, in a tuple
fn build_possible_games_nodes(
    teams: Vec<&Team>
) -> Vec<Game> {
    teams
        .iter()
        .combinations(2)
        .map(|combo| Game::new(combo[0], combo[1], 0))
        .collect()
}

// Needs to take Vec<Node> and return Vec<Edge>
fn build_remaining_points_edges<'a>(
    source_team: &'a Team,
    remaining_games: &'a Vec<&'a Game<'a>>,
    possible_games_nodes: &'a [Game<'a>],  // borrow slice instead of passing by value
) -> Vec<RemainingPointsSource<'a>> {
    possible_games_nodes
        .iter()
        .map(|possible_game| {
            let remaining = remaining_games
                .iter()
                .filter(|game| game.teams == possible_game.teams)
                .count() as u32 * 3;
            RemainingPointsSource::new(source_team, possible_game, remaining)
        })
        .collect()
}

fn build_infinite_capacity_edges<'a>(
    teams: Vec<&'a Team>,
    possible_games_nodes: Vec<Game<'a>>
) -> Vec<Edge<'a>> {
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

    infinite_capacity_edges
}

// needs to return Graph type
pub fn build_constraints<'a>(
    source: &'a Team,
    teams: Vec<&'a Team>,
    remaining_games: Vec<Game<'a>>,
) {
    // work out lifetimes to avoid cloning
    let possible_games_nodes = build_possible_games_nodes(teams.clone());
    let remaining_points_edges = build_remaining_points_edges(
        source,
        &remaining_games.iter().collect(),
        &possible_games_nodes,
    );
    let infinite_capacity_edges = build_infinite_capacity_edges(
        teams,
        possible_games_nodes
    );

    for ice in infinite_capacity_edges {
        println!("{:?}", ice.source_team);
        println!("{:?}", ice.target_team);
        println!("{:?}", ice.capacity);
    }
    // let remaining_distance_edges = vec![];


    // Graph { nodes: vec![], edges: vec![] }
}