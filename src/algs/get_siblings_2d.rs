///
/// Get adjacent vertices in a 2d matrix.
///
/// # config
/// ```json
/// {
///     "BFS get adjacent vertices": {
///         scope: "rust",
///         prefix: "get_siblings_2d",
///         description: "",
///         body:  /* code snippets insert here */,
///     },
/// }
/// ```
///

fn get_siblings_2d(index: (usize, usize), size: (usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = index;
    let mut adj = vec![];
    if i > 0 {
        adj.push((i - 1, j));
    }
    if i < size.0 - 1 {
        adj.push((i + 1, j));
    }
    if j > 0 {
        adj.push((i, j - 1));
    }
    if j < size.1 - 1 {
        adj.push((i, j + 1));
    }
    adj
}
