use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq, Debug)]
struct Stats {
    win: u8,
    loss: u8,
    draw: u8,
    points: u8,
    played: u8,
}
impl Stats {
    pub fn new() -> Self {
        Self {
            win: 0,
            loss: 0,
            draw: 0,
            points: 0,
            played: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut header = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut teams: HashMap<String, Stats> = HashMap::new();
    for l in match_results.lines() {
        let team_data: Vec<_> = l.split(';').collect();
        let home = team_data[0];
        let away = team_data[1];
        let result = team_data[2];
        let home = teams.entry(home.to_string()).or_insert(Stats::new());
        match result {
            "win" => {
                home.win += 1;
                home.points += 3;
            }
            "loss" => home.loss += 1,
            "draw" => {
                home.draw += 1;
                home.points += 1;
            }
            _ => (),
        }
        home.played += 1;
        let away = teams.entry(away.to_string()).or_insert(Stats::new());
        match result {
            "win" => away.loss += 1,
            "loss" => {
                away.win += 1;
                away.points += 3;
            }
            "draw" => {
                away.draw += 1;
                away.points += 1;
            }
            _ => (),
        }
        away.played += 1;
    }

    let mut sorted_teams: Vec<_> = teams.iter().collect();
    sorted_teams.sort_by(|a, b| {
        if b.1.points == a.1.points {
            a.0.cmp(&b.0)
        } else {
            b.1.points.cmp(&a.1.points)
        }
    });
   
    for t in sorted_teams {
        header.push_str(&format!(
            "\n{:31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            t.0, t.1.played, t.1.win, t.1.draw, t.1.loss, t.1.points
        ));
    }
    
    header
}