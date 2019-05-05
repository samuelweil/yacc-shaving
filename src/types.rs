use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
    Int(i64),
    Dbl(f64),
}

impl Number {
    fn to_float(&self) -> f64 {
        match self {
            Number::Int(i) => *i as f64,
            Number::Dbl(d) => *d,
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(i1), Number::Int(i2)) => Number::Int(i1 + i2),
            _ => Number::Dbl(self.to_float() + other.to_float()),
        }
    }
}

#[cfg(tests)]
mod tests {
    static TOL: f64 = 1e-6;
    static FLOAT_1: f64 = 3.4;
    static FLOAT_2: f64 = 7.272;
    static INT_1: i64 = 6;
    static INT_2: i64 = -7;
    static ZERO_INT: i64 = 0;
    static ZERO_FLT: f64 = 0.0;

    #[test]
    fn add_numbers() {
        let float1 = Number::Dbl(FLOAT_1);
        let float2 = Number::Dbl(FLOAT_2);

        let mut expected = Number::Dbl(FLOAT_1 + FLOAT_2);
        assert_eq!(float1 + float2, expected);

        let int2 = Number::Int(INT_2);
        expected = Number::Dbl(FLOAT_1 + INT_2 as f64);
        assert_eq!(float1 + int2, expected);

        expected = Number::Int(INT_1 + INT_2);
        let int1 = Number::Int(INT_1);
        assert_eq!(int1 + int2, expected);
    }
}
