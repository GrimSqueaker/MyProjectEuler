use std::fmt;

// Triangle
// $P_{3,n}=n(n+1)/2$
// $P_{3,n+1}-P_{3,n}=n+1$
// $1, 3, 6, 10, 15, \dots$
//
// Square
// $P_{4,n}=n^2=n(2n+0)/2$
// $P_{4,n+1}-P_{4,n}=2n+1$
// $1, 4, 9, 16, 25, \dots$
// 
// Pentagonal
// $P_{5,n}=n(3n-1)/2$
// $P_{5,n+1}-P_{5,n}=3n+1$
// $1, 5, 12, 22, 35, \dots$
// 
// Hexagonal
// $P_{6,n}=n(2n-1)=n(4n-2)/2$
// $P_{6,n+1}-P_{6,n}=4n+1$
// $1, 6, 15, 28, 45, \dots$
// 
// Heptagonal
// $P_{7,n}=n(5n-3)/2$
// $P_{7,n+1}-P_{7,n}=5n+1$
// $1, 7, 18, 34, 55, \dots$
// 
// Octagonal
// $P_{8,n}=n(3n-2)=n(6n-4)/2$
// $P_{8,n+1}-P_{8,n}=6n+1$
// $1, 8, 21, 40, 65, \dots$

// ...

// Polygonal
// $P_{i,n}=n((i-2)n+4-i)/2$
// $P_{i,n+1}-P_{i,n}=(i-2)n+1$


#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
pub struct Polygonal {
    pub gonality: u32,
    pub n: u32,
    pub value: u32,
}

impl Polygonal {
    pub fn new(gonality: u32, n: u32) -> Self {
        Polygonal {
            gonality: gonality,
            n: n, 
            value: n*((gonality-2)*n+4-gonality)/2
        }
    }

    pub fn prev(&self) -> Polygonal {
        Polygonal {
            gonality: self.gonality,
            n: self.n-1, 
            value: self.value - ((self.gonality-2)*(self.n-1)+1)
        }
    }

    pub fn next(&self) -> Polygonal {
        Polygonal {
            gonality: self.gonality,
            n: self.n+1, 
            value: self.value + (self.gonality-2)*self.n+1
        }
    }
}

impl fmt::Display for Polygonal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P_({}, {})={}", self.gonality, self.n, self.value)
    }
}

pub fn is_polygonal(gonality: u32, value: u32) -> Option<Polygonal> {
    is_polygonal_with_hint(value, &Polygonal::new(gonality, 1))
}

pub fn is_polygonal_with_hint(value: u32, hint: &Polygonal) -> Option<Polygonal> {
    if value == hint.value {
        Some(hint.clone())
    }
    else if value < hint.value {
        if value <= hint.prev().value {
            is_polygonal_with_hint(value, &hint.prev())
        }
        else {
            None
        }
    }
    else {
        if value >= hint.next().value {
            is_polygonal_with_hint(value, &hint.next())
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
        let p = Polygonal::new(3, 1);
        assert_eq!(p.n, 1);
        assert_eq!(p.value, 1);
        let p = Polygonal::new(3, 5);
        assert_eq!(p.n, 5);
        assert_eq!(p.value, 15);
    }

    #[test]
    fn test_next() {
        [(3,15), (4,25), (5,35), (6,45), (7,55), (8,65)]
            .iter()
            .for_each(|x| {
                let pol = Polygonal::new(x.0, 4).next();
                assert_eq!(pol.n, 5);
                assert_eq!(pol.value, x.1);
            });
    }


    #[test]
    fn test_prev() {
        [(3,3), (4,4), (5,5), (6,6), (7,7), (8,8)]
            .iter()
            .for_each(|x| {
                let pol = Polygonal::new(x.0, 3).prev();
                assert_eq!(pol.n, 2);
                assert_eq!(pol.value, x.1);
            });
    }

    #[test]
    fn test_is_polygonal() {
        (3..=8)
            .into_iter()
            .for_each(|x| {
                let pol = Polygonal::new(x, 5);
                assert_eq!(is_polygonal(x, pol.value).unwrap(), pol);
                assert_eq!(is_polygonal(x, pol.value+1), None);
            });
    }
}
