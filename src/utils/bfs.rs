use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::hash::Hash;

pub fn shortest<S, FN, IN, FG>(start: S, mut neighbors: FN, mut is_goal: FG) -> Option<usize>
where
    S: Hash + Eq + Clone,
    FN: FnMut(&S) -> IN,
    IN: IntoIterator<Item = S>,
    FG: FnMut(&S) -> bool,
{
    if is_goal(&start) {
        return Some(0);
    }
    let mut q: VecDeque<(S, usize)> = VecDeque::new();
    let mut seen: FxHashSet<S> = FxHashSet::default();
    seen.insert(start.clone());
    q.push_back((start, 0));
    while let Some((s, d)) = q.pop_front() {
        for n in neighbors(&s) {
            if !seen.insert(n.clone()) {
                continue;
            }
            if is_goal(&n) {
                return Some(d + 1);
            }
            q.push_back((n, d + 1));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_line() {
        let d = shortest(0i32, |&n| [n + 1].into_iter(), |&n| n == 5);
        assert_eq!(d, Some(5));
    }

    #[test]
    fn no_path() {
        let d = shortest(0i32, |_| std::iter::empty(), |&n| n == 1);
        assert_eq!(d, None);
    }

    #[test]
    fn start_is_goal() {
        let d = shortest(7i32, |_| std::iter::empty(), |&n| n == 7);
        assert_eq!(d, Some(0));
    }

    #[test]
    fn grid_4_neighbors() {
        let target = (3i32, 4i32);
        let d = shortest(
            (0i32, 0i32),
            |&(r, c)| [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)].into_iter(),
            |&p| p == target,
        );
        assert_eq!(d, Some(7));
    }
}
