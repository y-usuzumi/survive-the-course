use self::FastPowStrategy::*;

pub enum FastPowStrategy {
    Recursive,
    Iterative,
}

pub fn fast_pow(base: f64, exponent: i32, strategy: FastPowStrategy) -> f64 {
    match strategy {
        Recursive => fast_pow_recursive(base, exponent),
        Iterative => fast_pow_iterative(base, exponent),
    }
}

fn fast_pow_recursive(base: f64, exponent: i32) -> f64 {
    if exponent == 0 {
        if base == 0.0 {
            panic!("Math error: 0 to the power of 0");
        }
        return 1.0;
    }

    if exponent < 0 {
        return 1.0 / fast_pow_recursive(base, -exponent);
    }

    let (div, rem) = (exponent / 2, exponent % 2);
    let subresult = fast_pow_recursive(base, div);
    if rem == 0 {
        return subresult * subresult;
    }
    return subresult * subresult * base;
}

fn fast_pow_iterative(base: f64, exponent: i32) -> f64 {
    if exponent == 0 {
        if base == 0.0 {
            panic!("Math error: 0 to the power of 0");
        }
        return 1.0;
    }

    if exponent < 0 {
        return 1.0 / fast_pow_recursive(base, -exponent);
    }

    let mut result = 1.0;
    let mut bit = 1;
    let mut current_squared = base;
    while bit <= exponent {
        if exponent & bit > 0 {
            result *= current_squared;
        }
        current_squared *= current_squared;
        bit <<= 1;
    }
    if exponent & bit == 1 {
        result *= current_squared;
    }
    result
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_fast_pow_recursive_1() {
        assert!(
            (fast_pow(8.3, 78, FastPowStrategy::Recursive) - 4.876308857585593e+71).abs() < 1e65
        )
    }

    #[test]
    pub fn test_fast_pow_iterative_1() {
        assert!(
            (fast_pow(8.3, 78, FastPowStrategy::Iterative) - 4.876308857585593e+71).abs() < 1e65
        )
    }
}
