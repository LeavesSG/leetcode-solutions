///
/// Get adjacent vertices in a 2d matrix.
///
/// # config
/// ```json
/// {
///     "BFS get adjacent vertices": {
///         scope: "rust",
///         prefix: "get adj",
///         description: "",
///         body:  /* code snippets insert here */,
///     },
/// }
/// ```
///

fn get_adj(i: usize, j: usize, result: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let row_max = &result.len();
    let col_max = &result[0].len();

    let mut adj = vec![];
    if i > 0 {
        adj.push((i - 1, j));
    }
    if i < row_max - 1 {
        adj.push((i + 1, j));
    }
    if j > 0 {
        adj.push((i, j - 1));
    }
    if j < col_max - 1 {
        adj.push((i, j + 1));
    }
    adj
}
