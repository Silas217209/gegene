use std::fmt::Display;
use std::ops::Neg;
use num_format::utils::NanStr;

#[derive(Debug, Clone, Copy)]
pub enum Score {
    MateIn(i32),
    CP(i32),
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Score::MateIn(self_mate), Score::MateIn(other_mate)) => self_mate == other_mate,
            (Score::CP(self_score), Score::CP(other_score)) => self_score == other_score,
            _ => false,
        }
    }
}

impl Eq for Score {}

impl Neg for Score {
    type Output = Score;

    fn neg(self) -> Self::Output {
        match self {
            Score::MateIn(mate) => Score::MateIn(-mate),
            Score::CP(cp) => Score::CP(-cp),
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Score::CP(x), Score::CP(y)) => {
                x.cmp(y)
            }
            (Score::CP(_), Score::MateIn(y)) => {
                if *y < 0 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
            (Score::MateIn(x), Score::CP(y)) => {
                if *x < 0 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            (Score::MateIn(x), Score::MateIn(y)) => {
                if *x < 0 && *y < 0 {
                    y.cmp(x)
                } else if *x > 0 && *y > 0 {
                    y.cmp(x)
                } else {
                    x.cmp(y)
                }
            }
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::MateIn(mate) => write!(f, "mate {}", mate),
            Score::CP(cp) => write!(f, "cp {}", cp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greater_cp() {
        let a = Score::CP(-15);
        let b = Score::CP(25);
        assert!(a < b);
    }

    #[test]
    fn greater_mate() {
        let a = Score::MateIn(-5);
        let b = Score::MateIn(5);
        assert!(a < b);
    }

    #[test]
    fn greater_mate_vs_cp() {
        let a = Score::MateIn(-5); // you lose in 5 moves
        let b = Score::CP(5);
        assert!(a < b);
    }

    #[test]
    fn greater_mate_vs_cp_2() {
        let a = Score::MateIn(5); // you win in 5 moves
        let b = Score::CP(5);
        assert!(a > b);
    }

    #[test]
    fn cmp_mate_mate() {
        let a = Score::MateIn(-5); // you lose in 5 moves
        let b = Score::MateIn(-15); // you lose in 15 moves
        // losing in 15 moves is better than losing in 5 moves
        assert!(a < b);
    }

    #[test]
    fn cmp_mate_mate_2() {
        let a = Score::MateIn(5); // you win in 5 moves
        let b = Score::MateIn(15);
        assert!(a > b);
    }
}