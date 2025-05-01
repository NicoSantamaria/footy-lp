use std::collections::HashSet;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Team {
    name: String,
    points: u32,
}

struct RemainingPointsSource {
    source_team: Team,
    target_game: Game,
    remaining: u32,
}

impl RemainingPointsSource {
    fn new(
        source_team: &Team,
        target_game: &Game,
        remaining: u32
    ) -> RemainingPointsSource {
        RemainingPointsSource {
            source_team: source_team.clone(),
            target_game: target_game.clone(),
            remaining
        }
    }
}

// struct RemainingPointsSink {
//     source_team: Team,
//     remaining: u32,
// }

#[derive(Clone, Debug)]
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

fn build_constraints(
    team: Team,
    opponents: Vec<Team>,
    remaining_games: Vec<Game>
) {

    let possible_games: Vec<Game> = opponents
        .iter().combinations(2)
        .map(|combo: Vec<&Team>| Game::new(combo[0], combo[1], 0))
        .collect();

    let remaining_points_source: Vec<RemainingPointsSource> = possible_games
        .into_iter()
        .map(|possible_game: Game| {
            RemainingPointsSource::new(
                &team,
                &possible_game,
                remaining_games.iter()
                    .filter(|game| game.teams == possible_game.teams)
                    .count() as u32 * 3
            )
        })
        .collect();

    for remaining_points_source in remaining_points_source {
        println!("{:?}", remaining_points_source.source_team);
        println!("{:?}", remaining_points_source.target_game);
        println!("{:?}", remaining_points_source.remaining);
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


    build_constraints(teams[0].clone(), teams, games);
}