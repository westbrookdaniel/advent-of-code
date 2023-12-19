#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Self {
        let (name, rules) = s.trim().split_once('{').unwrap();
        let name = name.trim().to_string();
        let rules = rules
            .trim()
            .split(',')
            .map(|s| s.trim().strip_suffix('}').unwrap_or(s).trim())
            .map(Rule::from)
            .collect();
        Self { name, rules }
    }
}

#[derive(Debug, Clone)]
enum ProcessResult {
    Outcome(Outcome),
    Workflow(String),
}

#[derive(Debug)]
enum Rule {
    Outcome(Outcome),
    Workflow(String),
    ConditionToOutcome {
        outcome: Outcome,
        test: String,
        op: Op,
        value: u32,
    },
    ConditionToWorkflow {
        workflow: String,
        test: String,
        op: Op,
        value: u32,
    },
}

impl From<&str> for Rule {
    // m>1548:A OR A OR R OR foo<1283:ahj
    fn from(s: &str) -> Self {
        if s == "A" {
            return Self::Outcome(Outcome::Accept);
        }
        if s == "R" {
            return Self::Outcome(Outcome::Reject);
        }

        let opt = s.split_once(':');

        if let Some((left, right)) = opt {
            let (left, op, n) = if left.contains('<') {
                let (left, n) = left.split_once('<').unwrap();
                (left, Op::LessThan, n)
            } else {
                let (left, n) = left.split_once('>').unwrap();
                (left, Op::GreaterThan, n)
            };

            let left = left.trim();
            let right = right.trim();
            let value = n.trim().parse().unwrap();

            if right == "A" {
                return Self::ConditionToOutcome {
                    outcome: Outcome::Accept,
                    test: left.to_string(),
                    op,
                    value,
                };
            }

            if right == "R" {
                return Self::ConditionToOutcome {
                    outcome: Outcome::Reject,
                    test: left.to_string(),
                    op,
                    value,
                };
            }

            Self::ConditionToWorkflow {
                workflow: right.to_string(),
                test: left.to_string(),
                op,
                value,
            }
        } else {
            Self::Workflow(s.to_string())
        }
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Accept,
    Reject,
}

#[derive(Debug)]
enum Op {
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(&self, s: &String) -> u32 {
        let s = s.trim();
        match s {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Unknown part"),
        }
    }

    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d19p1.txt").unwrap();

    let (workflows, _) = input.split_once("\r\r\n\r\r").unwrap();

    let workflows = workflows
        .trim()
        .lines()
        .map(Workflow::from)
        .collect::<Vec<_>>();

    // find all parts that have Outcome::Accept or ConditionToOutcome with Outcome::Accept
    // and the indexes of the part
    let mut ends = vec![];
    for (i, workflow) in workflows.iter().enumerate() {
        for (j, rule) in workflow.rules.iter().enumerate() {
            match rule {
                Rule::Outcome(Outcome::Accept) => ends.push((i, j)),
                Rule::ConditionToOutcome {
                    outcome: Outcome::Accept,
                    test: _,
                    op: _,
                    value: _,
                } => ends.push((i, j)),
                _ => {}
            }
        }
    }

    // work from ends to find n of parts that lead to an Outcome::Accept
    let n = n_of_parts(&workflows, &ends);

    println!("{}", n);
}

// recursive find n of parts
fn n_of_parts(workflows: &Vec<Workflow>, ends: &Vec<(usize, usize)>) -> u64 {}
