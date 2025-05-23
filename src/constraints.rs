use std::{
    cell::RefCell,
    cmp,
    collections::HashSet,
    rc::Rc,
};
use itertools::Itertools;
use crate::graph::{
    EdgeKind, Edge, Node, foo
};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Team {
    pub name: String,
    pub points: i32,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub teams: HashSet<Team>,
    pub number: i32,
}

impl<'a> Game {
    pub fn new(
        team1: Team,
        team2: Team,
        number: i32
    ) -> Self {
        let teams: HashSet<Team> = HashSet::from([team1, team2]);
        Self { teams, number }
    }
}


fn build_possible_games_nodes(
    teams: Vec<Team>
) -> Vec<Rc<RefCell<Node>>> {
    teams.iter()
        .combinations(2)
        .map(|combo| {
            Node::new(HashSet::from([combo[0].clone(), combo[1].clone()]))
        })
        .collect()
}


fn build_possible_teams_nodes(
    teams: Vec<Team>
) -> Vec<Rc<RefCell<Node>>> {
    teams.iter()
        .map(|team| {
            Node::new(HashSet::from([team.clone()]))
        })
        .collect()
}

// needs to return Graph type
pub fn build_constraints(
    source: Team,
    score: i32,
    teams: Vec<Team>,
    remaining_games: Vec<Game>,
) -> Rc<RefCell<Node>> {
    let root = Node::from([source]);
    let possible_games_nodes = build_possible_games_nodes(teams.clone());
    let possible_teams_nodes = build_possible_teams_nodes(teams.clone());
    let sink = Node::new(HashSet::new());

    {
        // from root to pairings
        let mut mut_root = root.borrow_mut();
        mut_root.edges.extend({
            possible_games_nodes.iter()
                .map(|possible_game| {
                    Edge {
                        target: possible_game.clone(),
                        capacity: Some(remaining_games
                            .iter()
                            .filter(|game| game.teams == possible_game.borrow().datum)
                            .count() as i32 * 3),
                        kind: EdgeKind::FromSource
                    }
                })
                .collect::<Vec<Edge>>()
        });

        // from pairings to teams
        for possible_game in possible_games_nodes.iter() {
            let mut mut_possible_game = possible_game.borrow_mut();
            let game_datum = mut_possible_game.datum.clone();

            mut_possible_game.edges.extend({
                possible_teams_nodes.iter()
                    .filter(|possible_team| {
                        possible_team.borrow().datum.is_subset(&game_datum)
                    })
                    .map(|possible_team| {
                        Edge {
                            target: possible_team.clone(),
                            capacity: None,
                            kind: EdgeKind::InfiniteCapacity,
                        }
                    })
                    .collect::<Vec<Edge>>()
            });
        }

        // from teams to sink
        for possible_team in possible_teams_nodes.iter() {
            let mut mut_possible_team = possible_team.borrow_mut();
            let team_datum = mut_possible_team.datum.clone();

            mut_possible_team.edges.push({
                Edge {
                    target: sink.clone(),
                    capacity: team_datum.iter().next()
                        .map(|team| cmp::max(score - team.points, 0)),
                    kind: EdgeKind::ToSink
                }
            });
        };
    };

    root
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let score = 3;
        let teams: Vec<Team> = Vec::from([
            Team { name: "Argentina".into(), points: 0 },
            Team { name: "Mexico".into(), points: 1 },
            Team { name: "Poland".into(), points: 4 },
            Team { name: "Saudi Arabia".into(), points: 3 }
        ]);

        let teams_clone = teams.clone();

        let games: Vec<Game> = Vec::from([
            Game::new(teams_clone[0].clone(), teams_clone[1].clone(), 0),
            Game::new(teams_clone[0].clone(), teams_clone[2].clone(), 0),
            Game::new(teams_clone[1].clone(), teams_clone[3].clone(), 0),
        ]);

        let source = teams[0].clone();

        let root = build_constraints(source, score, teams, games);

        // ✅ Use the updated graph printer
        foo(&root);
    }
}