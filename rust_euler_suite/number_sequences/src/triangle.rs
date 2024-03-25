use std::fmt;

// Triangle   $T_n=n(n+1)/2$
//            $1, 3, 6, 10, 15, \dots$

// T_n - T_(n-1)
//   = n(n+1)/2 - (n-1)n/2
//   = (n^2 + n - n^2 + n)/2
//   = (n + n)/2
//   = n

#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub n: u32,
    pub value: u32,
}

impl Triangle {
    pub fn new(n: u32) -> Self {
        Triangle {
            n: n, 
            value: n*(n+1)/2
        }
    }

    pub fn prev(&self) -> Triangle {
        Triangle {
            n: self.n-1, 
            value: self.value - self.n
        }
    }

    pub fn next(&self) -> Triangle {
        Triangle {
            n: self.n+1, 
            value: self.value + self.n + 1
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.n, self.value)
    }
}

pub fn is_triangle(value: u32) -> Option<Triangle> {
    is_triangle_with_hint(value, &Triangle::new(1))
}

pub fn is_triangle_with_hint(value: u32, hint: &Triangle) -> Option<Triangle> {
    if value == hint.value {
        Some(hint.clone())
    }
    else if value < hint.value {
        if value <= hint.prev().value {
            is_triangle_with_hint(value, &hint.prev())
        }
        else {
            None
        }
    }
    else {
        if value >= hint.next().value {
            is_triangle_with_hint(value, &hint.next())
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
        let tri = Triangle::new(1);
        assert_eq!(tri.n, 1);
        assert_eq!(tri.value, 1);
        let tri = Triangle::new(5);
        assert_eq!(tri.n, 5);
        assert_eq!(tri.value, 15);
    }

    #[test]
    fn test_next() {
        let tri = Triangle::new(4).next();
        assert_eq!(tri.n, 5);
        assert_eq!(tri.value, 15);
    }

    #[test]
    fn test_prev() {
        let tri = Triangle::new(6).prev();
        assert_eq!(tri.n, 5);
        assert_eq!(tri.value, 15);
    }

    #[test]
    fn test_is_triangle() {
        let tri = Triangle::new(3);
        assert_eq!(is_triangle(tri.value).unwrap(), tri);
        assert_ne!(is_triangle(10).unwrap(), tri);
        assert_eq!(is_triangle(5), None);
        assert_ne!(is_triangle_with_hint(6, &tri), None);
        assert_eq!(is_triangle_with_hint(21, &tri), Some(Triangle::new(6)));
    }
}
