/**
 *  A breadth first search implemented for graph indices with `Copy`
 */
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
