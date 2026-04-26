#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Position<N> {
    pub x: N,
    pub y: N,
}
