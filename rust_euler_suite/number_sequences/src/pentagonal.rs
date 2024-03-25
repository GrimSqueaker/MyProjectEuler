use std::fmt;

// Pentagonal $P_n=n(3n - 1)/2$
//            $1, 5, 12, 22, 35, \dots$

// P_n - P_(n-1)
//   = n(3n-1)/2 - (n-1)(3(n-1)-1)/2
//   = ( 3n^2-n - (n-1)(3n-4) )/2
//   = ( 3n^2-n - (3n^2-4n-3n+4) )/2
//   = ( 6n - 4 )/2
//   = 3n - 2

#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
pub struct Pentagonal {
    pub n: u32,
    pub value: u32,
}

impl Pentagonal {
    pub fn new(n: u32) -> Self {
        Pentagonal {
            n: n, 
            value: n*(3*n-1)/2
        }
    }

    pub fn prev(&self) -> Pentagonal {
        Pentagonal {
            n: self.n-1, 
            value: self.value - (3*self.n-2)
        }
    }

    pub fn next(&self) -> Pentagonal {
        Pentagonal {
            n: self.n+1, 
            value: self.value + 3*self.n+1
        }
    }
}

impl fmt::Display for Pentagonal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.n, self.value)
    }
}

pub fn is_pentagonal(value: u32) -> Option<Pentagonal> {
    is_pentagonal_with_hint(value, &Pentagonal::new(1))
}

pub fn is_pentagonal_with_hint(value: u32, hint: &Pentagonal) -> Option<Pentagonal> {
    if value == hint.value {
        Some(hint.clone())
    }
    else if value < hint.value {
        if value <= hint.prev().value {
            is_pentagonal_with_hint(value, &hint.prev())
        }
        else {
            None
        }
    }
    else {
        if value >= hint.next().value {
            is_pentagonal_with_hint(value, &hint.next())
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pent = Pentagonal::new(1);
        assert_eq!(pent.n, 1);
        assert_eq!(pent.value, 1);
        let pent = Pentagonal::new(5);
        assert_eq!(pent.n, 5);
        assert_eq!(pent.value, 35);
    }

    #[test]
    fn test_next() {
        let pent = Pentagonal::new(4).next();
        assert_eq!(pent.n, 5);
        assert_eq!(pent.value, 35);
    }

    #[test]
    fn test_prev() {
        let pent = Pentagonal::new(6).prev();
        assert_eq!(pent.n, 5);
        assert_eq!(pent.value, 35);
    }

    #[test]
    fn test_is_pentagonal() {
        let pent = Pentagonal::new(6);
        assert_eq!(is_pentagonal(pent.value).unwrap(), pent);
        assert_ne!(is_pentagonal(5).unwrap(), pent);
        assert_eq!(is_pentagonal(3), None);
        assert_ne!(is_pentagonal_with_hint(5, &pent), None);
        assert_eq!(is_pentagonal_with_hint(145, &pent), Some(Pentagonal::new(10)));
    }
}
