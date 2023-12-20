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

    let mut precursors: HashMap<&str, Option<u64>> =
        [("tf", None), ("db", None), ("vq", None), ("ln", None)].into();

    for button_press in 1.. {
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
            // Check if we're sending a pulse to one of rx's precursors.
            if pulse == Pulse::High {
                if let Some(precursor) = precursors.get_mut(source) {
                    precursor.get_or_insert(button_press);

                    // If we've calculated the loop size for all precurors,
                    // calculate their product (part 2) and exit.
                    if precursors.values().all(Option::is_some) {
                        let part2: u64 = precursors.into_values().map(|o| o.unwrap()).product();
                        println!("part2 = {part2}");
                        std::process::exit(0);
                    }
                }
            }

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

        if button_press == 1000 {
            let part1 = low_sent * high_sent;
            println!("part1 = {part1}");
        }
    }
}
