use std::fmt;

// Hexagonal  $H_n=n(2n - 1)$
//            $1, 6, 15, 28, 45, \dots$

// H_n - H_(n-1)
//   = n(2n-1) - (n-1)(2(n-1)-1)
//   = 2n^2-n - (n-1)(2n-3)
//   = 2n^2-n - (2n^2-3n-2n+3)
//   = 2n^2-n - 2n^2+3n+2n-3
//   = 4n - 3

#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
pub struct Hexagonal {
    pub n: u32,
    pub value: u32,
}

impl Hexagonal {
    pub fn new(n: u32) -> Self {
        Hexagonal {
            n: n, 
            value: n*(2*n-1)
        }
    }

    pub fn prev(&self) -> Hexagonal {
        Hexagonal {
            n: self.n-1, 
            value: self.value - (4*self.n-3)
        }
    }

    pub fn next(&self) -> Hexagonal {
        Hexagonal {
            n: self.n+1, 
            value: self.value + 4*self.n+1
        }
    }
}

impl fmt::Display for Hexagonal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.n, self.value)
    }
}

pub fn is_hexagonal(value: u32) -> Option<Hexagonal> {
    is_hexagonal_with_hint(value, &Hexagonal::new(1))
}

pub fn is_hexagonal_with_hint(value: u32, hint: &Hexagonal) -> Option<Hexagonal> {
    if value == hint.value {
        Some(hint.clone())
    }
    else if value < hint.value {
        if value <= hint.prev().value {
            is_hexagonal_with_hint(value, &hint.prev())
        }
        else {
            None
        }
    }
    else {
        if value >= hint.next().value {
            is_hexagonal_with_hint(value, &hint.next())
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
        let hex = Hexagonal::new(1);
        assert_eq!(hex.n, 1);
        assert_eq!(hex.value, 1);
        let hex = Hexagonal::new(5);
        assert_eq!(hex.n, 5);
        assert_eq!(hex.value, 45);
    }

    #[test]
    fn test_next() {
        let hex = Hexagonal::new(4).next();
        assert_eq!(hex.n, 5);
        assert_eq!(hex.value, 45);
    }

    #[test]
    fn test_prev() {
        let hex = Hexagonal::new(6).prev();
        assert_eq!(hex.n, 5);
        assert_eq!(hex.value, 45);
    }

    #[test]
    fn test_is_hexagonal() {
        let hex = Hexagonal::new(5);
        assert_eq!(is_hexagonal(hex.value).unwrap(), hex);
        assert_ne!(is_hexagonal(15).unwrap(), hex);
        assert_eq!(is_hexagonal(3), None);
        assert_eq!(is_hexagonal_with_hint(5, &hex), None);
        assert_eq!(is_hexagonal_with_hint(28, &hex), Some(Hexagonal::new(4)));
    }
}

//            $1, 6, 15, 28, 45, \dots$