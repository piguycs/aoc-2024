use std::collections::HashSet;

// i used ai to write this whole function
// shame on me... :'(
pub fn find_largest_cluster(positions: Vec<(i32, i32)>) -> usize {
    let positions_set: HashSet<_> = positions.into_iter().collect();
    let mut visited: HashSet<_> = HashSet::new();

    // Define the directions using itertools for brevity
    let directions = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    // Helper function to perform DFS
    fn dfs(
        position: (i32, i32),
        positions_set: &HashSet<(i32, i32)>,
        visited: &mut HashSet<(i32, i32)>,
        directions: &[(i32, i32)],
    ) -> usize {
        let mut stack = vec![position];
        let mut cluster_size = 0;

        while let Some(pos) = stack.pop() {
            if visited.insert(pos) {
                // Try inserting, if it returns true, it was not visited yet
                cluster_size += 1;
                for &(dx, dy) in directions.iter() {
                    let neighbor = (pos.0 + dx, pos.1 + dy);
                    if positions_set.contains(&neighbor) && !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        cluster_size
    }

    // Find the largest cluster size using Itertools
    positions_set
        .iter()
        .filter_map(|&pos| {
            if !visited.contains(&pos) {
                Some(dfs(pos, &positions_set, &mut visited, &directions))
            } else {
                None
            }
        })
        .max() // Get the maximum cluster size
        .unwrap_or(0) // In case no clusters are found (though this shouldn't happen)
}
