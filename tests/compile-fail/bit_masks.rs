#![feature(plugin)]
#![plugin(clippy)]

const THREE_BITS : i64 = 7;
const EVEN_MORE_REDIRECTION : i64 = THREE_BITS;

#[deny(bad_bit_mask)]
#[allow(ineffective_bit_mask, identity_op)]
fn main() {
    let x = 5;

    x & 0 == 0; //~ERROR &-masking with zero
    x & 1 == 1; //ok, distinguishes bit 0
    x & 1 == 0; //ok, compared with zero
    x & 2 == 1; //~ERROR
    x | 0 == 0; //ok, equals x == 0 (maybe warn?)
    x | 1 == 3; //ok, equals x == 2 || x == 3
    x | 3 == 3; //ok, equals x <= 3
    x | 3 == 2; //~ERROR

    x & 1 > 1; //~ERROR
    x & 2 > 1; // ok, distinguishes x & 2 == 2 from x & 2 == 0
    x & 2 < 1; // ok, distinguishes x & 2 == 2 from x & 2 == 0
    x | 1 > 1; // ok (if a bit silly), equals x > 1
    x | 2 > 1; //~ERROR
    x | 2 <= 2; // ok (if a bit silly), equals x <= 2

    // this also now works with constants
    x & THREE_BITS == 8; //~ERROR
    x | EVEN_MORE_REDIRECTION < 7; //~ERROR

    0 & x == 0; //~ERROR
    1 | x > 1;

    // and should now also match uncommon usage
    1 < 2 | x; //~ERROR
    2 == 3 | x; //~ERROR
    1 == x & 2; //~ERROR

    x | 1 > 2; // no error, because we allowed ineffective bit masks
    ineffective();
}

#[deny(ineffective_bit_mask)]
#[allow(bad_bit_mask)]
fn ineffective() {
    let x = 5;

    x | 1 > 2; //~ERROR
    x | 1 < 3; //~ERROR
    x | 1 <= 3; //~ERROR
    x | 1 >= 2; //~ERROR
}
