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

    fn bfs_owned<T, P>(start: T, get_adj: &P) -> Vec<T>
    where
        T: Copy + Eq + std::hash::Hash,
        P: Fn(T) -> Vec<T>,
    {
        let mut queue = vec![start];
        let mut checked = std::collections::HashMap::new();
        while !queue.is_empty() {
            match queue.pop() {
                Some(item) => {
                    checked.insert(item, true);
                    get_adj(item)
                        .into_iter()
                        .for_each(|adj| match checked.get(&adj) {
                            None => {
                                queue.push(adj);
                            }
                            Some(_) => (),
                        });
                }
                None => (),
            }
        }
        checked.into_iter().map(|(key, _)| key).collect()
    }

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
