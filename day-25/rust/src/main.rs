use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../test_input.txt");

    let mut map: BTreeMap<&str, HashSet<&str>> = [].into();

    for line in input.lines() {
        let (a, v) = line.split_once(": ").unwrap();

        for b in v.split(' ') {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
        }
    }

    let indices: Indices = map
        .keys()
        .copied()
        .enumerate()
        .map(|(i, node)| (node, i))
        .collect();

    let (_distances, paths) = floyd_warshall(&map, &indices);

    let mut tally2: HashMap<Edge, usize> = [].into();
    for node_a in map.keys() {
        for node_b in map.keys() {
            if node_a != node_b {
                let path = fw_path(&paths, &indices, node_a, node_b);
                for edge in path.windows(2) {
                    let edge = canonical(edge[0], edge[1]);
                    *tally2.entry(edge).or_default() += 1;
                }
            }
        }
    }

    let mut sorted: Vec<(Edge, usize)> = tally2.into_iter().collect();
    sorted.sort_by(|(_, a), (_, b)| a.cmp(b).reverse());

    let edges_to_remove: Vec<Edge> = sorted.into_iter().take(3).map(|(edge, _)| edge).collect();

    let (a, b) = edges_to_remove.first().copied().unwrap();
    let a = graph_size(&map, a, &edges_to_remove);
    let b = graph_size(&map, b, &edges_to_remove);

    println!("part1 = {}", a * b);
}

type Map<'a> = BTreeMap<&'a str, HashSet<&'a str>>;
type Edge<'a> = (&'a str, &'a str);
type Path<'a> = Vec<&'a str>;

fn canonical<'a>(a: &'a str, b: &'a str) -> Edge<'a> {
    match a < b {
        true => (a, b),
        false => (b, a),
    }
}

fn graph_size(map: &Map, start: &str, bad_edges: &[Edge]) -> usize {
    let mut queue: VecDeque<&str> = [start].into();
    let mut seen: HashSet<&str> = [].into();

    while let Some(node) = queue.pop_front() {
        if !seen.insert(node) {
            continue;
        }

        for neighbor in map.get(node).unwrap() {
            if bad_edges.contains(&canonical(node, neighbor)) {
                continue;
            }

            queue.push_back(neighbor);
        }
    }

    seen.len()
}

type Indices<'a> = HashMap<&'a str, usize>;
type Distances = Vec<Vec<usize>>;
type Paths<'a> = Vec<Vec<&'a str>>;

/// Floyd Warshall w/ path reconstruction.
fn floyd_warshall<'a>(map: &'a Map, indices: &Indices<'a>) -> (Distances, Paths<'a>) {
    let mut distances: Distances = vec![vec![usize::MAX; map.len()]; map.len()];
    let mut paths: Paths = vec![vec![""; map.len()]; map.len()];

    let mut edges = HashSet::<(&str, &str)>::default();
    for (k, v) in map.iter() {
        for w in v {
            edges.insert(canonical(k, w));
        }
    }

    for edge in edges {
        let (a, b) = edge;

        let a_idx = *indices.get(a).unwrap();
        let b_idx = *indices.get(b).unwrap();

        distances[a_idx][b_idx] = 1;
        distances[b_idx][a_idx] = 1;
        paths[a_idx][b_idx] = a;
        paths[b_idx][a_idx] = b;
    }

    for (vertex, &i) in indices.iter() {
        distances[i][i] = 0;
        paths[i][i] = vertex;
    }

    for k in 0..map.len() {
        for i in 0..map.len() {
            for j in 0..map.len() {
                let to_k = distances[i][k];
                let from_k = distances[k][j];
                let direct = distances[i][j];
                let through = to_k.checked_add(from_k).unwrap_or(usize::MAX);

                if direct > through {
                    distances[i][j] = through;
                    distances[j][i] = through;

                    paths[i][j] = paths[k][j];
                    paths[j][i] = paths[k][j];
                }
            }
        }
    }

    assert!(distances.iter().all(|d| d.iter().all(|x| *x != usize::MAX)));
    assert!(paths
        .iter()
        .all(|path| path.iter().all(|node| !node.is_empty())));

    (distances, paths)
}

/// Reconstruct path from Floyd-Warshall data.
fn fw_path<'a>(
    paths: &Paths<'a>,
    indices: &Indices<'a>,
    source: &'a str,
    mut dest: &'a str,
) -> Path<'a> {
    let source_idx = *indices.get(source).unwrap();
    let mut dest_idx = *indices.get(dest).unwrap();

    let mut path = vec![dest];
    while source != dest {
        dest = paths[source_idx][dest_idx];
        dest_idx = *indices.get(dest).unwrap();

        path.insert(0, dest);
    }

    path
}
