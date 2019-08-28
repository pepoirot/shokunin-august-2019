use std::option::Option;
use std::fmt::Display;
use std::fmt::Formatter;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Developer {
    Evan, Jessie, John, Matt, Sarah
}

#[derive(Clone)]
pub struct Ranking {
    order: Vec<Developer> // from best to worst
}

impl Ranking {
    pub fn alphabetical() -> Ranking {
        Ranking {
            order: vec![Developer::Evan, Developer::Jessie, Developer::John, Developer::Matt, Developer::Sarah]
        }
    }

    pub fn random() -> Ranking {
        let mut order = Ranking::alphabetical().order;
        let mut rng = rand::thread_rng();
        order.shuffle(&mut rng);
        Ranking { order }
    }

    pub fn get_best(&self) -> Option<&Developer> {
        self.order.first()
    }

    pub fn get_worst(&self) -> Option<&Developer> {
        self.order.last()
    }

    pub fn get_relative_rank(&self, dev: &Developer, other_dev: &Developer) -> isize {
        let rank_dev = self.order.iter().position(|&d| d == *dev).unwrap() as isize;
        let rank_other_dev = self.order.iter().position(|&d| d == *other_dev).unwrap() as isize;
        return rank_other_dev - rank_dev;
    }

    pub fn shuffle(&self, count: u64) -> Ranking {
        let mut rng = rand::thread_rng();
        let mut order = self.order.clone();
        for _ in 0..count {
            let p1 = rng.gen_range(0, order.len());
            let p2 = rng.gen_range(0, order.len());
            order.swap(p1, p2);
        }

        Ranking { order }
    }
}

impl Display for Ranking {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self.order.len() {
            0 => write!(formatter, "Empty ranking"),
            1 => write!(formatter, "{:?}", self.order[0]),
            _ => {
                write!(formatter, "{:?} (best) -> ", self.order[0]).ok();
                for developer in self.order.iter().skip(1).take(self.order.len() - 2) {
                    write!(formatter, "{:?} -> ", developer).ok();
                }
                write!(formatter, "{:?} (worst)", self.order[self.order.len() - 1])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranking_get_best() {
        assert_eq!(Ranking::alphabetical().get_best(), Some(&Developer::Evan));
    }

    #[test]
    fn test_ranking_get_worst() {
        assert_eq!(Ranking::alphabetical().get_worst(), Some(&Developer::Sarah));
    }

    #[test]
    fn test_ranking_get_relative_rank() {
        let alphabetical_ranking = Ranking::alphabetical();
        assert_eq!(alphabetical_ranking.get_relative_rank(&Developer::Evan, &Developer::Sarah), 4);
        assert_eq!(alphabetical_ranking.get_relative_rank(&Developer::Evan, &Developer::Jessie), 1);
        assert_eq!(alphabetical_ranking.get_relative_rank(&Developer::Jessie, &Developer::Matt), 2);
        assert_eq!(alphabetical_ranking.get_relative_rank(&Developer::Matt, &Developer::Jessie), -2);
        assert_eq!(alphabetical_ranking.get_relative_rank(&Developer::Matt, &Developer::Matt), 0);
    }

    #[test]
    fn test_ranking_random() {
        let alphabetical_ranking = Ranking::alphabetical();
        let random_ranking = Ranking::random();
        assert_eq!(alphabetical_ranking.order.len(), random_ranking.order.len());
    }

    #[test]
    fn test_ranking_formatting() {
        let ranking = Ranking::alphabetical();
        assert_eq!(&format!("{}", ranking), "Evan (best) -> Jessie -> John -> Matt -> Sarah (worst)");
    }

}
