pub type V3 = [i32; 3];

pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn neg(a: V3) -> V3 {
    [-a[0], -a[1], -a[2]]
}

pub fn scale(a: V3, k: i32) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}

pub fn dot(a: V3, b: V3) -> i32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arith() {
        assert_eq!(add([1, 2, 3], [4, 5, 6]), [5, 7, 9]);
        assert_eq!(sub([4, 5, 6], [1, 2, 3]), [3, 3, 3]);
        assert_eq!(neg([1, -2, 3]), [-1, 2, -3]);
        assert_eq!(scale([1, 2, 3], 4), [4, 8, 12]);
        assert_eq!(dot([1, 2, 3], [4, 5, 6]), 32);
    }
}
