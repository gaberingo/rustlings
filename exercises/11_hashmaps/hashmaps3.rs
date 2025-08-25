// Daftar skor (satu per baris) dari pertandingan sepak bola diberikan. Setiap baris memiliki format
// "<nama_tim_1>,<nama_tim_2>,<gol_tim_1>,<gol_tim_2>"
// Contoh: "England,France,4,2" (England mencetak 4 gol, France 2 gol).
//
// Anda harus membuat tabel skor yang berisi nama tim, total gol yang dicetak tim,
// dan total gol yang kebobolan tim tersebut.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default, Debug)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.
        
        let teamscores1 = scores.entry(team_1_name).or_default();
        teamscores1.goals_scored += team_1_score;
        teamscores1.goals_conceded += team_2_score;
        
        let teamscores2 = scores.entry(team_2_name).or_default();
        teamscores2.goals_scored += team_2_score;
        teamscores2.goals_conceded += team_1_score;



    }

    scores
}

fn main() {
    // You can optionally experiment here.
    let scores = build_scores_table("England,France,3,2");
    dbg!(scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
