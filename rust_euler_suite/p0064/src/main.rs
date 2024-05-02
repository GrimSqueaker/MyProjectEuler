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
    pub starting_number: u32,
    pub periodic_part: Vec<u32>,
}

fn main() {
}

fn get_continued_fraction_representation_for_square_root_of(number: u32) -> ContinuedFractionRepresentation {
    ContinuedFractionRepresentation{starting_number: 0, periodic_part: vec![]}
}

fn get_period_length(cf: &ContinuedFractionRepresentation) -> usize {
    cf.periodic_part.len()
}

fn get_floor_of_root_n_plus_x_over_y(n: u32, x: i32, y: i32) -> i32 {
    // returns floor( (sqrt(n)+x)/y )
    // finds largest a such that (ay-x)^2 <= n
    0
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
    fn test_get_period_length() {
        assert_eq!(get_period_length(&ContinuedFractionRepresentation{
            starting_number: 2, 
            periodic_part: vec![]}), 0);
        assert_eq!(get_period_length(&ContinuedFractionRepresentation{
            starting_number: 1, 
            periodic_part: vec![2]}), 1);
        assert_eq!(get_period_length(&ContinuedFractionRepresentation{
            starting_number: 2, 
            periodic_part: vec![2, 4]}), 2);
    }

    #[test]
    fn test_get_floor_of_root_n_plus_x_over_y() {
        assert_eq!(get_floor_of_root_n_plus_x_over_y(16, 0, 1), 4);
    }
}

