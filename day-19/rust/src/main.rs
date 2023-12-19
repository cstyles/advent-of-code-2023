use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Operation {
    LessThan,
    GreaterThan,
}

impl Operation {
    fn parse(c: char) -> Self {
        match c {
            '<' => Self::LessThan,
            '>' => Self::GreaterThan,
            _ => unreachable!("bad input: {c}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rule<'a> {
    var: char,
    operation: Operation,
    value: u64,
    result: Outcome<'a>,
}

impl<'a> Rule<'a> {
    fn parse(line: &'a str) -> Rule<'a> {
        let mut chars = line.chars();

        let var = chars.next().unwrap();
        let operation = Operation::parse(chars.next().unwrap());

        let (first, result) = line.split_once(':').unwrap();
        let value = first[2..].parse().unwrap();
        let result = Outcome::parse(result);

        Self {
            var,
            operation,
            value,
            result,
        }
    }

    fn outcome(&self, part: Part) -> Option<Outcome> {
        let op = match self.operation {
            Operation::LessThan => |a, b| a < b,
            Operation::GreaterThan => |a, b| a > b,
        };

        let category = match self.var {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!("TODO"),
        };

        op(category, self.value).then_some(self.result)
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome<'a> {
    Accept,
    Reject,
    Redirect(&'a str),
}

impl<'a> Outcome<'a> {
    fn parse(string: &'a str) -> Outcome<'a> {
        match string {
            "A" => Self::Accept,
            "R" => Self::Reject,
            var => Self::Redirect(var),
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    default: Outcome<'a>,
}

impl<'a> Workflow<'a> {
    fn parse(line: &'a str) -> Workflow<'a> {
        let (name, rules) = line.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap();

        let (rules, default) = rules.rsplit_once(',').unwrap();
        let default = Outcome::parse(default);

        let rules = rules.split(',').map(Rule::parse).collect();

        Self {
            name,
            rules,
            default,
        }
    }

    fn accepts(&self, workflows: &HashMap<&'_ str, Self>, part: Part) -> bool {
        for rule in self.rules.iter() {
            match rule.outcome(part) {
                Some(Outcome::Accept) => return true,
                Some(Outcome::Reject) => return false,
                Some(Outcome::Redirect(name)) => {
                    return workflows.get(name).unwrap().accepts(workflows, part)
                }
                None => continue,
            }
        }

        match self.default {
            Outcome::Accept => true,
            Outcome::Reject => false,
            Outcome::Redirect(name) => workflows.get(name).unwrap().accepts(workflows, part),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn parse(string: &str) -> Self {
        let string = &string[1..string.len() - 1]; // trim curly braces

        let mut values = string.split(',').map(|string| string[2..].parse().unwrap());

        let x = values.next().unwrap();
        let m = values.next().unwrap();
        let a = values.next().unwrap();
        let s = values.next().unwrap();

        Self { x, m, a, s }
    }

    fn total(self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<&str, Workflow<'_>> = workflows
        .lines()
        .map(Workflow::parse)
        .map(|workflow| (workflow.name, workflow))
        .collect();

    let parts: Vec<Part> = parts.lines().map(Part::parse).collect();

    // dbg!(workflows);
    // dbg!(parts);

    let mut part1 = 0;
    for part in parts {
        let workflow = workflows.get("in").unwrap();
        if workflow.accepts(&workflows, part) {
            part1 += part.total();
        }
    }

    println!("part1 = {part1}");
}
