use super::rankings::{Developer, Ranking};

// Find a ranking of the developers based on a set of predicates
//
// The solver follows a simple Hill Climbing algorithm:
// https://en.wikipedia.org/wiki/Hill_climbing
pub fn find_ranking(max_attempts: u64) -> Result<Ranking, String> {
    let mut attempts = 0;
    let mut ranking = Ranking::random();

    let mut best_score = std::u64::MAX;
    let mut best_ranking = ranking.clone();

    while attempts <= max_attempts {
        let score = compute_score(&ranking);

        if score == 0 {
            return Ok(ranking);
        } else if score <= best_score {
            best_score = score;
            best_ranking = ranking;
        }

        ranking = best_ranking.shuffle(1);
        attempts += 1;
    }

    Err(format!("no solution reached within {} attempts (best found was: {})", max_attempts, best_ranking))
}

fn compute_score(ranking: &Ranking) -> u64 {
    let mut failed_predicates = 0;
    if let Some(Developer::Jessie) = ranking.get_best() {
        failed_predicates += 1;
    }
    if let Some(Developer::Evan) = ranking.get_worst() {
        failed_predicates += 1;
    }
    if let Some(Developer::John) = ranking.get_best() {
        failed_predicates += 1;
    } else if let Some(Developer::John) = ranking.get_worst() {
        failed_predicates += 1;
    }
    if ranking.get_relative_rank(&Developer::Sarah, &Developer::Evan) <= 0 {
        failed_predicates += 1;
    }
    if ranking.get_relative_rank(&Developer::Matt, &Developer::John).abs() <= 1 {
        failed_predicates += 1;
    }
    if ranking.get_relative_rank(&Developer::John, &Developer::Evan).abs() <= 1 {
        failed_predicates += 1;
    }
    return failed_predicates;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_score() {
        assert_eq!(compute_score(&Ranking::alphabetical()), 2);
    }

    #[test]
    fn test_find_ranking_in_10000_attempts() {
        let result = find_ranking(10000);
        match result {
            Err(_) => panic!("No solution found"),
            Ok(ranking) => {
                assert_eq!(&format!("{}", ranking), "Sarah (best) -> John -> Jessie -> Evan -> Matt (worst)");
            }
       }
    }
}
