// Integer right triangles
//
// Problem 39
//
// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.
//
// {20,48,52}, {24,45,51}, {30,40,50}
//
// For which value of p ≤ 1000, is the number of solutions maximised?

fn is_right_triangle(a: usize, b: usize, c: usize) -> bool {
    if a*a + b*b == c*c {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_right_triangle() {
        assert_eq!(is_right_triangle(3, 4, 5), true);
        assert_eq!(is_right_triangle(20, 48, 52), true);
        assert_eq!(is_right_triangle(24, 45, 51), true);
        assert_eq!(is_right_triangle(3, 3, 4), false);
    }
}


fn main() {
    const MAX: usize = 1001;

    let mut num_triangle = [0; MAX];

    for c in 3..MAX {
        for b in 1..c {
            for a in 1..b {
                if is_right_triangle(a, b, c) && (a+b+c < MAX) {
                    num_triangle[(a+b+c) as usize] += 1;
                }
            }
        }
    }

    let mut found = 0;
    for i in 1usize..MAX {
        if num_triangle[i] > 0 {
            println!("Perimeter {} has {} integer triangles", i, num_triangle[i]);
        }

        if num_triangle[i] > num_triangle[found] {
            found = i;
        }
    }
    println!("Maximal number of integer triangles {} for perimeter {}", num_triangle[found], found);
}

// Perimeter 12 has 1 integer triangles
// Perimeter 24 has 1 integer triangles
// Perimeter 30 has 1 integer triangles
// Perimeter 36 has 1 integer triangles
// Perimeter 40 has 1 integer triangles
// Perimeter 48 has 1 integer triangles
// Perimeter 56 has 1 integer triangles
// Perimeter 60 has 2 integer triangles
// Perimeter 70 has 1 integer triangles
// Perimeter 72 has 1 integer triangles
// Perimeter 80 has 1 integer triangles
// Perimeter 84 has 2 integer triangles
// Perimeter 90 has 2 integer triangles
// Perimeter 96 has 1 integer triangles
// Perimeter 108 has 1 integer triangles
// Perimeter 112 has 1 integer triangles
// Perimeter 120 has 3 integer triangles
// Perimeter 126 has 1 integer triangles
// Perimeter 132 has 2 integer triangles
// Perimeter 140 has 1 integer triangles
// Perimeter 144 has 2 integer triangles
// Perimeter 150 has 1 integer triangles
// Perimeter 154 has 1 integer triangles
// Perimeter 156 has 1 integer triangles
// Perimeter 160 has 1 integer triangles
// Perimeter 168 has 3 integer triangles
// Perimeter 176 has 1 integer triangles
// Perimeter 180 has 3 integer triangles
// Perimeter 182 has 1 integer triangles
// Perimeter 192 has 1 integer triangles
// Perimeter 198 has 1 integer triangles
// Perimeter 200 has 1 integer triangles
// Perimeter 204 has 1 integer triangles
// Perimeter 208 has 1 integer triangles
// Perimeter 210 has 2 integer triangles
// Perimeter 216 has 1 integer triangles
// Perimeter 220 has 1 integer triangles
// Perimeter 224 has 1 integer triangles
// Perimeter 228 has 1 integer triangles
// Perimeter 234 has 1 integer triangles
// Perimeter 240 has 4 integer triangles
// Perimeter 252 has 3 integer triangles
// Perimeter 260 has 1 integer triangles
// Perimeter 264 has 2 integer triangles
// Perimeter 270 has 2 integer triangles
// Perimeter 276 has 1 integer triangles
// Perimeter 280 has 3 integer triangles
// Perimeter 286 has 1 integer triangles
// Perimeter 288 has 2 integer triangles
// Perimeter 300 has 2 integer triangles
// Perimeter 306 has 1 integer triangles
// Perimeter 308 has 1 integer triangles
// Perimeter 312 has 2 integer triangles
// Perimeter 320 has 1 integer triangles
// Perimeter 324 has 1 integer triangles
// Perimeter 330 has 2 integer triangles
// Perimeter 336 has 3 integer triangles
// Perimeter 340 has 1 integer triangles
// Perimeter 348 has 1 integer triangles
// Perimeter 350 has 1 integer triangles
// Perimeter 352 has 1 integer triangles
// Perimeter 360 has 4 integer triangles
// Perimeter 364 has 1 integer triangles
// Perimeter 372 has 1 integer triangles
// Perimeter 374 has 1 integer triangles
// Perimeter 378 has 1 integer triangles
// Perimeter 380 has 1 integer triangles
// Perimeter 384 has 1 integer triangles
// Perimeter 390 has 2 integer triangles
// Perimeter 392 has 1 integer triangles
// Perimeter 396 has 3 integer triangles
// Perimeter 400 has 1 integer triangles
// Perimeter 408 has 2 integer triangles
// Perimeter 416 has 1 integer triangles
// Perimeter 418 has 1 integer triangles
// Perimeter 420 has 5 integer triangles
// Perimeter 432 has 2 integer triangles
// Perimeter 440 has 2 integer triangles
// Perimeter 442 has 1 integer triangles
// Perimeter 444 has 1 integer triangles
// Perimeter 448 has 1 integer triangles
// Perimeter 450 has 2 integer triangles
// Perimeter 456 has 2 integer triangles
// Perimeter 462 has 2 integer triangles
// Perimeter 468 has 2 integer triangles
// Perimeter 476 has 1 integer triangles
// Perimeter 480 has 4 integer triangles
// Perimeter 490 has 1 integer triangles
// Perimeter 492 has 1 integer triangles
// Perimeter 494 has 1 integer triangles
// Perimeter 504 has 4 integer triangles
// Perimeter 510 has 2 integer triangles
// Perimeter 516 has 1 integer triangles
// Perimeter 520 has 2 integer triangles
// Perimeter 528 has 3 integer triangles
// Perimeter 532 has 1 integer triangles
// Perimeter 540 has 3 integer triangles
// Perimeter 544 has 1 integer triangles
// Perimeter 546 has 2 integer triangles
// Perimeter 552 has 2 integer triangles
// Perimeter 560 has 3 integer triangles
// Perimeter 564 has 1 integer triangles
// Perimeter 570 has 2 integer triangles
// Perimeter 572 has 1 integer triangles
// Perimeter 576 has 2 integer triangles
// Perimeter 588 has 2 integer triangles
// Perimeter 594 has 1 integer triangles
// Perimeter 598 has 1 integer triangles
// Perimeter 600 has 3 integer triangles
// Perimeter 608 has 1 integer triangles
// Perimeter 612 has 2 integer triangles
// Perimeter 616 has 2 integer triangles
// Perimeter 624 has 3 integer triangles
// Perimeter 630 has 4 integer triangles
// Perimeter 636 has 1 integer triangles
// Perimeter 640 has 1 integer triangles
// Perimeter 644 has 1 integer triangles
// Perimeter 646 has 1 integer triangles
// Perimeter 648 has 1 integer triangles
// Perimeter 650 has 1 integer triangles
// Perimeter 660 has 5 integer triangles
// Perimeter 672 has 4 integer triangles
// Perimeter 680 has 2 integer triangles
// Perimeter 684 has 2 integer triangles
// Perimeter 690 has 2 integer triangles
// Perimeter 696 has 1 integer triangles
// Perimeter 700 has 2 integer triangles
// Perimeter 702 has 1 integer triangles
// Perimeter 704 has 1 integer triangles
// Perimeter 708 has 1 integer triangles
// Perimeter 714 has 1 integer triangles
// Perimeter 720 has 6 integer triangles
// Perimeter 728 has 2 integer triangles
// Perimeter 732 has 1 integer triangles
// Perimeter 736 has 1 integer triangles
// Perimeter 744 has 1 integer triangles
// Perimeter 748 has 1 integer triangles
// Perimeter 750 has 1 integer triangles
// Perimeter 756 has 4 integer triangles
// Perimeter 760 has 2 integer triangles
// Perimeter 768 has 1 integer triangles
// Perimeter 770 has 2 integer triangles
// Perimeter 780 has 4 integer triangles
// Perimeter 782 has 1 integer triangles
// Perimeter 784 has 1 integer triangles
// Perimeter 792 has 3 integer triangles
// Perimeter 798 has 1 integer triangles
// Perimeter 800 has 2 integer triangles
// Perimeter 804 has 1 integer triangles
// Perimeter 810 has 2 integer triangles
// Perimeter 816 has 2 integer triangles
// Perimeter 828 has 2 integer triangles
// Perimeter 832 has 1 integer triangles
// Perimeter 836 has 1 integer triangles
// Perimeter 840 has 8 integer triangles
// Perimeter 850 has 1 integer triangles
// Perimeter 852 has 1 integer triangles
// Perimeter 858 has 1 integer triangles
// Perimeter 864 has 3 integer triangles
// Perimeter 870 has 2 integer triangles
// Perimeter 874 has 1 integer triangles
// Perimeter 876 has 1 integer triangles
// Perimeter 880 has 3 integer triangles
// Perimeter 882 has 1 integer triangles
// Perimeter 884 has 1 integer triangles
// Perimeter 888 has 1 integer triangles
// Perimeter 896 has 1 integer triangles
// Perimeter 900 has 4 integer triangles
// Perimeter 910 has 2 integer triangles
// Perimeter 912 has 2 integer triangles
// Perimeter 918 has 2 integer triangles
// Perimeter 920 has 2 integer triangles
// Perimeter 924 has 5 integer triangles
// Perimeter 928 has 1 integer triangles
// Perimeter 930 has 1 integer triangles
// Perimeter 936 has 3 integer triangles
// Perimeter 948 has 1 integer triangles
// Perimeter 950 has 1 integer triangles
// Perimeter 952 has 2 integer triangles
// Perimeter 960 has 4 integer triangles
// Perimeter 966 has 1 integer triangles
// Perimeter 972 has 1 integer triangles
// Perimeter 980 has 1 integer triangles
// Perimeter 984 has 1 integer triangles
// Perimeter 986 has 1 integer triangles
// Perimeter 988 has 1 integer triangles
// Perimeter 990 has 4 integer triangles
// Perimeter 992 has 1 integer triangles
// Perimeter 996 has 1 integer triangles
// Perimeter 1000 has 1 integer triangles
// Maximal number of integer triangles 8 for perimeter 840