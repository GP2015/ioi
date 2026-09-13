use crate::passed_map::PassedMap;

#[derive(Clone, Debug, Default)]
pub struct Info {
    pub hits_p: bool,
    pub steps: u32,
    pub best_in: bool,
}

pub fn create(graph: &[u32], n: u32, p: u32) -> &[Info] {
    let len = graph.len();
    let mut found = vec![false; len].into_boxed_slice();
    let mut info = vec![Info::default(); len].into_boxed_slice();

    let mut pmap = PassedMap::new(n);

    for head in 0..len {
        if found[head] {
            continue;
        }

        let mut curr_head = u32::try_from(head).unwrap();
        let mut step_counter = 0;

        pmap.clear();

        loop {
            if curr_head >> 1 == p {
                for (r_state, r_steps) in pmap.iter() {
                    found[r_state as usize] = true;
                    info[r_state as usize] = Info {
                        hits_p: true,
                        steps: step_counter - r_steps,
                        best_in: curr_head & 1 == 1,
                    };
                }
                break;
            }

            if found[curr_head as usize] {
                if info[curr_head as usize].hits_p {
                    for (r_state, r_steps) in pmap.iter() {
                        found[r_state as usize] = true;
                        info[r_state as usize] = Info {
                            hits_p: true,
                            steps: step_counter - r_steps + info[curr_head as usize].steps,
                            best_in: info[curr_head as usize].best_in,
                        };
                    }
                } else {
                    for (r_state, _) in pmap.iter() {
                        found[r_state as usize] = true;
                    }
                }
                break;
            }

            if pmap.contains_state(curr_head) {
                for (r_state, _) in pmap.iter() {
                    found[r_state as usize] = true;
                }
                break;
            }

            pmap.insert(curr_head, step_counter);

            curr_head = graph[curr_head as usize];
            step_counter += 1;
        }
    }

    Box::leak(info)
}
