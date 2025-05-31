pub mod frequency;

#[derive(Debug)]
enum Frequency {
    Zero = -1,
    Special = 0,
    Annual = 1,
    Semiannual = 2,
    Triannual = 3,
    Quarterly = 4,
    Monthly=12,
    Continuous=99,
}
