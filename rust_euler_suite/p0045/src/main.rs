
use lib_number_sequences::hexagonal::Hexagonal;
use lib_number_sequences::pentagonal::Pentagonal;
use lib_number_sequences::triangle::Triangle;

// `P0045`: https://projecteuler.net/problem=45
//
// Triangle, pentagonal, and hexagonal numbers are generated by the following formulae:
// Triangle   $T_n=n(n+1)/2$
//            $1, 3, 6, 10, 15, \dots$
// Pentagonal $P_n=n(3n - 1)/2$
//            $1, 5, 12, 22, 35, \dots$
// Hexagonal  $H_n=n(2n - 1)$
//            $1, 6, 15, 28, 45, \dots$
//   	
// It can be verified that $T_{285} = P_{165} = H_{143} = 40755$ .
// 
// Find the next triangle number that is also pentagonal and hexagonal.

// T_n - T_(n-1)
//   = n(n+1)/2 - (n-1)n/2
//   = (n^2 + n - n^2 + n)/2
//   = (n + n)/2
//   = n

// P_n - P_(n-1)
//   = n(3n-1)/2 - (n-1)(3(n-1)-1)/2
//   = ( 3n^2-n - (n-1)(3n-4) )/2
//   = ( 3n^2-n - (3n^2-4n-3n+4) )/2
//   = ( 6n - 4 )/2
//   = 3n - 2

// H_n - H_(n-1)
//   = n(2n-1) - (n-1)(2(n-1)-1)
//   = 2n^2-n - (n-1)(2n-3)
//   = 2n^2-n - (2n^2-3n-2n+3)
//   = 2n^2-n - 2n^2+3n+2n-3
//   = 4n - 3


struct Triple {
    pub tria: Triangle,
    pub penta: Pentagonal,
    pub hexa: Hexagonal,
}

fn main() {
    // idea:
    // - create a 3-tuple of $T_{285} = P_{165} = H_{143} = 40755$.
    // - increase H_{143} to H_{144}
    // - until all three have the same value
    //   - increase the smallest number of the 3-tuple
    //   - check if the values are the same
    //   - if yes: output solution and stop

    let mut trip = Triple {
        tria: Triangle::new(285),
        penta: Pentagonal::new(165),
        hexa: Hexagonal::new(143),
    };

    loop {
        if trip.tria.value < trip.penta.value && trip.tria.value < trip.hexa.value {
            trip.tria = trip.tria.next();
        }
        else if trip.penta.value < trip.hexa.value {
            trip.penta = trip.penta.next();
        }
        else {
            trip.hexa = trip.hexa.next();
        }

        if trip.tria.value == trip.penta.value && trip.tria.value == trip.hexa.value {
            println!("Found 3-tuple: T{} P{} H{}", trip.tria, trip.penta, trip.hexa);
            break;
        }
    }
}

// Found 3-tuple: T(55385, 1533776805) P(31977, 1533776805) H(27693, 1533776805)
