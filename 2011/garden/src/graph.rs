// States are encoded as u32s.
// Whether or not the best trail was taken to the current fountain
// is stored as the lowest bit (1 for yes and 0 for no)
// and the fountain number is shifted 1 bit to the left to make room
// (this cannot overflow the u32 due to the problem constraints).

// With the above encoding, states can all fit
// next to each other in contiguous memory
// without leaving any gaps between them,
// hence this graph.

// Due to the above, the index of a state in the graph
// representing a given `fountain` will be
// (`fountain` * 2) + 1 if the best trail was taken in or
// `fountain` * 2 if a different trail was taken in.

// Trails in `r` are ordered by beauty (precedence),
// hence why the below formula works.

pub fn create(n: u32, r: &[[u32; 2]]) -> &'static [u32] {
    let len = n as usize * 2;
    let mut graph = vec![0; len].into_boxed_slice();
    let mut found = vec![false; len].into_boxed_slice();

    for pair in r {
        let double = pair.map(|v| v * 2);
        let found_best_out = double.map(|d| found[d as usize]);
        let found_runner_out = double.map(|d| found[d as usize + 1]);

        for (i, j) in [(0, 1), (1, 0)] {
            if !found_runner_out[i] {
                let head = double[i] as usize + usize::from(found_best_out[i]);
                found[head] = true;
                graph[head] = double[j] + u32::from(!found_best_out[j]);
            }
        }
    }

    for idx in (0..len).step_by(2) {
        if !found[idx + 1] {
            graph[idx + 1] = graph[idx];
        }
    }

    Box::leak(graph)
}
