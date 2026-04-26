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
