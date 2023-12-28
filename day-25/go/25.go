package main

import (
	"cmp"
	"fmt"
	"os"
	"slices"
	"strings"
)

type Set[T comparable] map[T]bool
type Graph = map[string]Set[string]
type Indices = map[string]int
type Distances = [][]uint
type Paths = [][]string
type Edge = [2]string
type Path = []string

func graph_insert(graph Graph, key string, value string) {
	set, exists := graph[key]
	if exists {
		set[value] = true
	} else {
		graph[key] = Set[string]{value: true}
	}
}

func canonical(a string, b string) Edge {
	if a < b {
		return Edge{a, b}
	} else {
		return Edge{b, a}
	}
}

// Floyd Warshall w/ path reconstruction.
func floyd_warshall(graph Graph, indices Indices) (Distances, Paths) {
	distances := make(Distances, len(graph))
	paths := make(Paths, len(graph))
	for i := 0; i < len(graph); i++ {
		paths[i] = make([]string, len(graph))
		distances[i] = make([]uint, len(graph))
		for j := range distances {
			// Can't use math.MaxInt b/c no checked addition :(
			distances[i][j] = 999999
		}
	}

	edges := make(Set[Edge])
	for k, v := range graph {
		for w := range v {
			edges[canonical(k, w)] = true
		}
	}

	for edge := range edges {
		a := edge[0]
		b := edge[1]

		a_idx := indices[a]
		b_idx := indices[b]

		distances[a_idx][b_idx] = 1
		distances[b_idx][a_idx] = 1
		paths[a_idx][b_idx] = a
		paths[b_idx][a_idx] = b
	}

	for vertex, i := range indices {
		distances[i][i] = 0
		paths[i][i] = vertex
	}

	for k := 0; k < len(graph); k++ {
		for i := 0; i < len(graph); i++ {
			for j := 0; j < len(graph); j++ {
				to_k := distances[i][k]
				from_k := distances[k][j]
				direct := distances[i][j]
				through := to_k + from_k

				if direct > through {
					distances[i][j] = through
					distances[j][i] = through
					paths[i][j] = paths[k][j]
					paths[j][i] = paths[k][j]
				}
			}
		}
	}

	return distances, paths
}

// Reconstruct path from Floyd-Warshall data.
func fw_path(paths Paths, indices Indices, source string, dest string) Path {
	source_idx := indices[source]
	dest_idx := indices[dest]

	path := Path{dest}
	for source != dest {
		dest = paths[source_idx][dest_idx]
		dest_idx = indices[dest]
		path = append(Path{dest}, path...)
	}

	return path
}

func main() {
	// input_bytes, _ := os.ReadFile("../test_input.txt")
	input_bytes, _ := os.ReadFile("../input.txt")
	input := strings.TrimSpace(string(input_bytes))

	graph := make(Graph)

	for _, line := range strings.Split(input, "\n") {
		a, v, _ := strings.Cut(line, ": ")

		for _, b := range strings.Split(v, " ") {
			graph_insert(graph, a, b)
			graph_insert(graph, b, a)
		}
	}

	keys := make([]string, 0, len(graph))
	for k := range graph {
		keys = append(keys, k)
	}

	slices.Sort(keys)

	indices := make(Indices, len(keys))
	for i, key := range keys {
		indices[key] = i
	}

	_, paths := floyd_warshall(graph, indices)

	tallies := make(map[Edge]uint)
	for node_a := range graph {
		for node_b := range graph {
			if node_a != node_b {
				path := fw_path(paths, indices, node_a, node_b)
				for i := 0; i < len(path)-1; i++ {
					edge := canonical(path[i], path[i+1])
					if t, ok := tallies[edge]; ok {
						tallies[edge] = t + 1
					} else {
						tallies[edge] = 1
					}
				}
			}
		}
	}

	tallies_keys := make([]Edge, 0, len(tallies))
	for key := range tallies {
		tallies_keys = append(tallies_keys, key)
	}

	// Sort keys (edges) by tally count in descending order
	slices.SortFunc(tallies_keys, func(a Edge, b Edge) int {
		return -cmp.Compare(tallies[a], tallies[b])
	})

	edges_to_remove := tallies_keys[0:3]
	a := graph_size(graph, edges_to_remove[0][0], edges_to_remove)
	b := graph_size(graph, edges_to_remove[0][1], edges_to_remove)
	fmt.Println(a * b)
}

func graph_size(graph Graph, start string, bad_edges []Edge) int {
	stack := []string{start}
	seen := Set[string]{}

	for len(stack) != 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if _, ok := seen[node]; ok {
			continue
		} else {
			seen[node] = true
		}

		for neighbor := range graph[node] {
			if slices.Contains(bad_edges, canonical(node, neighbor)) {
				continue
			}

			stack = append(stack, neighbor)
		}
	}

	return len(seen)
}
