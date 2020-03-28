pub fn decode_zigzag(input: u32) -> i64 {
    return (input as i64 >> 1) ^ (-(input as i64 & 1));
}

pub fn signed_area(dots: &Vec<[i64; 2]>) -> f64 {
    let mut area = 0;
    let n = dots.len();

    for i in 0..(n - 1) {
        area += dots[i][0] * dots[i + 1][1];
    }

    area += dots[n - 1][0] * dots[0][1];
    for i in 0..(n - 1) {
        area -= dots[i + 1][0] * dots[i][1];
    }

    area -= dots[0][0] * dots[n - 1][1];

    area as f64 / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_area() {
        let mut dots = vec![[3, 4], [5, 11], [12, 8], [9, 5], [5, 6]];
        assert_eq!(signed_area(&dots), -30.0);

        dots = vec![[1, 1], [2, 1], [2, 2], [1, 2]];
        assert_eq!(signed_area(&dots), 1.0);
    }
}