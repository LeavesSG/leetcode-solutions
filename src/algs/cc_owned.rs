///
/// A connected components algorithm implemented for graph indices with `Copy`
/// Use the implementation of bfs_owned
///

fn cc_owned<T, P>(vertices: Vec<T>, get_adj: &P) -> Vec<(T, i32)>
where
    T: Copy + Eq + std::hash::Hash,
    P: Fn(T) -> Vec<T>,
{
    let mut checked = std::collections::HashMap::new();
    let mut index = 0;
    vertices
        .into_iter()
        .for_each(|vertex| match checked.get(&vertex) {
            None => {
                let b = bfs_owned(vertex, &get_adj);
                b.into_iter().for_each(|item| {
                    checked.insert(item, index);
                });
                index += 1;
            }
            Some(_) => (),
        });
    checked.into_iter().collect()
}