pub fn iter_neighbours_4(n: usize, m: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    let n = n as isize;
    let m = m as isize;
    let i = i as isize;
    let j = j as isize;
    [(0, 1), (-1, 0), (0, -1), (1, 0)]
        .into_iter()
        .filter(move |&(x, y)| i + x >= 0 && i + x < n && j + y >= 0 && j + y < m)
        .map(move |(x, y)| ((i + x) as usize, (j + y) as usize))
}

pub fn iter_neighbours_4d(n: usize, m: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    let n = n as isize;
    let m = m as isize;
    let i = i as isize;
    let j = j as isize;
    [(-1, 1), (-1, -1), (1, -1), (1, 1)]
        .into_iter()
        .filter(move |&(x, y)| i + x >= 0 && i + x < n && j + y >= 0 && j + y < m)
        .map(move |(x, y)| ((i + x) as usize, (j + y) as usize))
}

pub fn iter_neighbours_8(n: usize, m: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    let n = n as isize;
    let m = m as isize;
    let i = i as isize;
    let j = j as isize;
    [(0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)]
        .into_iter()
        .filter(move |&(x, y)| i + x >= 0 && i + x < n && j + y >= 0 && j + y < m)
        .map(move |(x, y)| ((i + x) as usize, (j + y) as usize))
}
