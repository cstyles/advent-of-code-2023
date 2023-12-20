use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum NodeKind<'a> {
    FlipFlop { on: bool },
    Conjunction { inputs: HashMap<&'a str, Pulse> },
    Broacaster,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    destinations: Vec<&'a str>,
    kind: NodeKind<'a>,
}

impl<'a> Node<'a> {
    fn parse(string: &'a str) -> Node<'a> {
        let (node, destinations) = string.split_once(" -> ").unwrap();

        let destinations = destinations.split(", ").collect();

        let (name, kind) = match node.chars().next().unwrap() {
            'b' => ("broadcaster", NodeKind::Broacaster),
            '%' => (&node[1..], NodeKind::FlipFlop { on: false }),
            '&' => (&node[1..], NodeKind::Conjunction { inputs: [].into() }),
            c => unreachable!("bad input: {c}"),
        };

        Self {
            name,
            destinations,
            kind,
        }
    }

    fn process(&mut self, source: &'a str, pulse: Pulse) -> Vec<Signal<'a>> {
        match &mut self.kind {
            NodeKind::FlipFlop { on } => {
                if pulse == Pulse::Low {
                    *on = !*on;
                    if *on {
                        self.destinations
                            .iter()
                            .map(|dest| Signal {
                                source: self.name,
                                dest,
                                pulse: Pulse::High,
                            })
                            .collect()
                    } else {
                        self.destinations
                            .iter()
                            .map(|dest| Signal {
                                source: self.name,
                                dest,
                                pulse: Pulse::Low,
                            })
                            .collect()
                    }
                } else {
                    vec![]
                }
            }
            NodeKind::Conjunction { inputs } => {
                let last = inputs.get_mut(source).unwrap();
                *last = pulse;
                if inputs.iter().all(|(_, pulse)| *pulse == Pulse::High) {
                    self.destinations
                        .iter()
                        .map(|dest| Signal {
                            source: self.name,
                            dest,
                            pulse: Pulse::Low,
                        })
                        .collect()
                } else {
                    self.destinations
                        .iter()
                        .map(|dest| Signal {
                            source: self.name,
                            dest,
                            pulse: Pulse::High,
                        })
                        .collect()
                }
            }
            NodeKind::Broacaster => todo!(),
        }
    }
}

struct Signal<'a> {
    source: &'a str,
    dest: &'a str,
    pulse: Pulse,
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    // let input = include_str!("../../test_input2.txt");
    let input = include_str!("../../input.txt");

    let mut nodes: HashMap<&str, Node> = input
        .lines()
        .map(Node::parse)
        .map(|node| (node.name, node))
        .collect();

    // Maps a node name to its list of inputs.
    let mut input_map: HashMap<&str, Vec<&str>> = [].into();
    for node in nodes.values() {
        for destination in node.destinations.iter() {
            let inputs = input_map.entry(destination).or_default();
            inputs.push(node.name);
        }
    }

    // Update inputs for Conjunction nodes.
    for node in nodes.values_mut() {
        if let NodeKind::Conjunction { inputs, .. } = &mut node.kind {
            let ugh = input_map.get(node.name).unwrap();
            let ugh = ugh.iter().map(|input| (*input, Pulse::Low));
            inputs.extend(ugh);
        }
    }

    // dbg!(nodes);

    let mut queue: VecDeque<Signal> = [].into();
    let mut low_sent = 0;
    let mut high_sent = 0;

    for button_press in 0.. {
        // Pressing the button also sends a Low pulse.
        low_sent += 1;

        // Start by broadcasting Low to all of broadcaster's destinations.
        let broadcaster = nodes.get("broadcaster").unwrap();
        for dest in broadcaster.destinations.iter() {
            queue.push_back(Signal {
                source: "broadcaster",
                dest,
                pulse: Pulse::Low,
            });
        }

        while let Some(Signal {
            source,
            dest,
            pulse,
        }) = queue.pop_front()
        {
            if ["ln", "db", "vq", "tf"].contains(&source) && pulse == Pulse::High {
                println!("{source} sent {pulse:?} to {dest} at bp #{button_press}");
            }

            // println!("{} sent {:?} to {}", source, pulse, dest);
            match pulse {
                Pulse::High => high_sent += 1,
                Pulse::Low => low_sent += 1,
            };

            if dest == "output" {
                continue;
            }

            if let Some(node) = nodes.get_mut(dest) {
                let results = node.process(source, pulse);
                queue.extend(results);
            } else if pulse == Pulse::Low {
                println!("part2 = {}", button_press + 1);
            }
        }
    }

    let part1 = dbg!(low_sent) * dbg!(high_sent);
    println!("part1 = {part1}");
}
