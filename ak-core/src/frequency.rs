pub mod frequency;

#[derive(Debug)]
enum frequency {
    zero = -1,
    special = 0,
    annual = 1,
    semiannual = 2,
    triannual = 3,
    quarterly = 4,
    monthly=12,
    continuous=99,
}
