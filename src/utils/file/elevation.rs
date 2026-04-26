use super::position::Position;

pub fn to_elevation_map<N, I>(input: I) -> (Vec<Vec<N>>, Position<usize>, Position<usize>)
where
    N: From<u8>,
    I: IntoIterator<Item = String>,
{
    let mut elevation: Vec<Vec<N>> = Vec::new();
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    for (y, line) in input.into_iter().enumerate() {
        let mut row: Vec<N> = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            let mut ascii = chr as u8;
            if chr == 'S' {
                start = Position { x, y };
                ascii = b'a';
            } else if chr == 'E' {
                end = Position { x, y };
                ascii = b'z';
            }
            row.push(ascii.into());
        }
        elevation.push(row);
    }
    (elevation, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_start_end_to_a_z() {
        let lines = ["SbE"].map(String::from);
        let (m, s, e) = to_elevation_map::<u8, _>(lines);
        assert_eq!(s, Position { x: 0, y: 0 });
        assert_eq!(e, Position { x: 2, y: 0 });
        assert_eq!(m, vec![vec![b'a', b'b', b'z']]);
    }
}
