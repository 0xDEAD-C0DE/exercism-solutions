use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq, Debug)]
struct Stats {
    win: u8,
    loss: u8,
    draw: u8,
    points: u8,
    played: u8,
}
impl Stats{
    pub fn new() -> Self {
        Self {
            win: 0,
            loss: 0,
            draw: 0,
            points: 0,
            played: 0,
        }
    }
    pub fn win(&mut self) {
    self.win += 1;
    self.points += 3;
    self.played += 1;
    }
    pub fn loss(&mut self) {
        self.loss += 1;
    self.played += 1;
    }
    pub fn draw(&mut self) {
        self.draw += 1;
        self.points += 1;
    self.played += 1;
    }
}
pub fn tally(match_results: &str) -> String {
    let header = "Team                           | MP |  W |  D |  L |  P\n";
    if match_results.is_empty() {
       return "Team                           | MP |  W |  D |  L |  P".to_string();
    }
    let data = match_results.split(&[';', '\n']).collect::<Vec<&str>>();
    let data = data.chunks(3).collect::<Vec<&[&str]>>();
    let mut output = String::new();
    let mut teams: HashMap<String, Stats> = HashMap::new();
    for d in &data {
        for dd in d.iter() {
            if *dd == "win" || *dd == "loss"  || *dd == "draw" {
                continue;
            }
            teams.insert(dd.to_string(), Stats::new());
        }
    }
        for d in &data {
            for dd in d.iter() {
                if *dd == "win" {
                    teams.get_mut(d[0]).unwrap().win();
                    teams.get_mut(d[1]).unwrap().loss();
                }
                if *dd == "loss" {
                    teams.get_mut(d[0]).unwrap().loss();
                    teams.get_mut(d[1]).unwrap().win();
                }
                if *dd == "draw" {
                    teams.get_mut(d[0]).unwrap().draw();
                    teams.get_mut(d[1]).unwrap().draw();
                }

            }
        } 

    let mut sorted_teams: Vec<_> = teams.iter().collect();
    sorted_teams.sort_by(|a,b| {
        if b.1.points == a.1.points {
           a.0.cmp(&b.0) 

        } else {
            b.1.points.cmp(&a.1.points)
        }
    });
    output.push_str(&header);
    for t in sorted_teams {
       output.push_str(&format!("{:31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}\n", 
            t.0, t.1.played, t.1.win, t.1.draw, t.1.loss, t.1.points));
   }
    output.pop();
    output
}