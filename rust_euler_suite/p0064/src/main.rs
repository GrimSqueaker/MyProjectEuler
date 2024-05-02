// 
// <p>All square roots are periodic when written as continued fractions and can be written in the form:</p>
// 
// $\displaystyle \quad \quad \sqrt{N}=a_0+\frac 1 {a_1+\frac 1 {a_2+ \frac 1 {a3+ \dots}}}$
// 
// <p>For example, let us consider $\sqrt{23}:$</p>
// $\quad \quad \sqrt{23}=4+\sqrt{23}-4=4+\frac 1 {\frac 1 {\sqrt{23}-4}}=4+\frac 1  {1+\frac{\sqrt{23}-3}7}$
// 
// <p>If we continue we would get the following expansion:</p>
// 
// $\displaystyle \quad \quad \sqrt{23}=4+\frac 1 {1+\frac 1 {3+ \frac 1 {1+\frac 1 {8+ \dots}}}}$
// 
// <p>The process can be summarised as follows:</p>
// <p>
// $\quad \quad a_0=4, \frac 1 {\sqrt{23}-4}=\frac {\sqrt{23}+4} 7=1+\frac {\sqrt{23}-3} 7$<br>
// $\quad \quad a_1=1, \frac 7 {\sqrt{23}-3}=\frac {7(\sqrt{23}+3)} {14}=3+\frac {\sqrt{23}-3} 2$<br>
// $\quad \quad a_2=3, \frac 2 {\sqrt{23}-3}=\frac {2(\sqrt{23}+3)} {14}=1+\frac {\sqrt{23}-4} 7$<br>
// $\quad \quad a_3=1, \frac 7 {\sqrt{23}-4}=\frac {7(\sqrt{23}+4)} 7=8+\sqrt{23}-4$<br>
// $\quad \quad a_4=8, \frac 1 {\sqrt{23}-4}=\frac {\sqrt{23}+4} 7=1+\frac {\sqrt{23}-3} 7$<br>
// $\quad \quad a_5=1, \frac 7 {\sqrt{23}-3}=\frac {7 (\sqrt{23}+3)} {14}=3+\frac {\sqrt{23}-3} 2$<br>
// 
// $\quad \quad a_6=3, \frac 2 {\sqrt{23}-3}=\frac {2(\sqrt{23}+3)} {14}=1+\frac {\sqrt{23}-4} 7$<br>
// $\quad \quad a_7=1, \frac 7 {\sqrt{23}-4}=\frac {7(\sqrt{23}+4)} {7}=8+\sqrt{23}-4$<br>
// </p>
// 
// <p>It can be seen that the sequence is repeating. For conciseness, we use the notation $\sqrt{23}=[4;(1,3,1,8)]$, to indicate that the block (1,3,1,8) repeats indefinitely.</p>
// 
// <p>The first ten continued fraction representations of (irrational) square roots are:</p>
// <p>
// $\quad \quad \sqrt{2}=[1;(2)]$, period=$1$<br>
// $\quad \quad \sqrt{3}=[1;(1,2)]$, period=$2$<br>
// $\quad \quad \sqrt{5}=[2;(4)]$, period=$1$<br>
// $\quad \quad \sqrt{6}=[2;(2,4)]$, period=$2$<br>
// $\quad \quad \sqrt{7}=[2;(1,1,1,4)]$, period=$4$<br>
// $\quad \quad \sqrt{8}=[2;(1,4)]$, period=$2$<br>
// $\quad \quad \sqrt{10}=[3;(6)]$, period=$1$<br>
// $\quad \quad \sqrt{11}=[3;(3,6)]$, period=$2$<br>
// $\quad \quad \sqrt{12}=[3;(2,6)]$, period=$2$<br>
// $\quad \quad \sqrt{13}=[3;(1,1,1,1,6)]$, period=$5$
// </p>
// <p>Exactly four continued fractions, for $N \le 13$, have an odd period.</p>
// <p>How many continued fractions for $N \le 10\,000$ have an odd period?</p>
// 


#[derive(PartialEq,Debug)]
struct ContinuedFractionRepresentation {
    pub starting_number: i64,
    pub periodic_part: Vec<i64>,
}

impl ContinuedFractionRepresentation {
    pub fn period_length(&self) -> usize {
        self.periodic_part.len()
    }
}


#[derive(PartialEq,Debug)]
struct PRnPxOy {
    // expression of the form p(sqrt(n)+x)/y
    p: i64,
    n: i64,
    x: i64,
    y: i64,
}

impl PRnPxOy {
    pub fn floor(&self) -> i64 {
        // returns floor( p(sqrt(n)+x)/y )
        // finds largest a such that (ay/p-x)^2 <= n
        0
    }

    pub fn reciprocal(&self) -> PRnPxOy {
        // assumes that the number is in (0,1)
        PRnPxOy{p: 1, n: 1, x: 0, y: 1}
    }

    pub fn cancelled(&self) -> PRnPxOy {
        // assumes that the number is in (0,1)
        PRnPxOy{p: 1, n: 1, x: 0, y: 1}
    }

    
}

fn main() {
}

fn get_continued_fraction_representation_for_square_root_of(number: i64) -> ContinuedFractionRepresentation {
    ContinuedFractionRepresentation{starting_number: 0, periodic_part: vec![]}
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_continued_fraction_representation_for_square_root_of() {
        assert_eq!(get_continued_fraction_representation_for_square_root_of(2),
            ContinuedFractionRepresentation{
                starting_number: 1, 
                periodic_part: vec![2]});
        assert_eq!(get_continued_fraction_representation_for_square_root_of(4),
            ContinuedFractionRepresentation{
                starting_number: 2, 
                periodic_part: vec![]});
        assert_eq!(get_continued_fraction_representation_for_square_root_of(13),
            ContinuedFractionRepresentation{
                starting_number: 3, 
                periodic_part: vec![1,1,1,1,6]});
    }

    #[test]
    fn test_period_length() {
        assert_eq!(ContinuedFractionRepresentation{
            starting_number: 2, 
            periodic_part: vec![]}.period_length(), 0);
        assert_eq!(ContinuedFractionRepresentation{
            starting_number: 1, 
            periodic_part: vec![2]}.period_length(), 1);
        assert_eq!(ContinuedFractionRepresentation{
            starting_number: 2, 
            periodic_part: vec![2, 4]}.period_length(), 2);
    }

    #[test]
    fn test_floor() {
        assert_eq!(PRnPxOy{p: 1, n: 16, x: 0, y: 1}.floor(), 4);
        assert_eq!(PRnPxOy{p: 1, n: 23, x: 0, y: 1}.floor(), 4);
        assert_eq!(PRnPxOy{p: 1, n: 23, x: 0, y: 1}.floor(), 4);
    }

    #[test]
    fn test_reciprocal() {
        assert_eq!(PRnPxOy{p: 1, n: 23, x: -3, y: 7}.reciprocal(), PRnPxOy{p: 7, n: 23, x: 3, y: 14});
    }

    #[test]
    fn test_cancelled() {
        assert_eq!(PRnPxOy{p: 2, n: 23, x: -3, y: 6}.cancelled(), PRnPxOy{p: 1, n: 23, x: -3, y: 3});
        assert_eq!(PRnPxOy{p: 4, n: 23, x: -3, y: 6}.cancelled(), PRnPxOy{p: 2, n: 23, x: -3, y: 3});
    }
}

