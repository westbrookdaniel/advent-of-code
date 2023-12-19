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

impl Workflow {
    fn process(&self, part: &Part) -> ProcessResult {
        for rule in self.rules.iter() {
            match rule {
                Rule::Outcome(outcome) => return ProcessResult::Outcome(outcome.clone()),
                Rule::Workflow(str) => return ProcessResult::Workflow(str.clone()),
                Rule::ConditionToOutcome {
                    outcome,
                    test,
                    op,
                    value,
                } => {
                    let test = match op {
                        Op::GreaterThan => part.get(test) > *value,
                        Op::LessThan => part.get(test) < *value,
                    };
                    if test {
                        return ProcessResult::Outcome(outcome.clone());
                    }
                }
                Rule::ConditionToWorkflow {
                    workflow,
                    test,
                    op,
                    value,
                } => {
                    let test = match op {
                        Op::GreaterThan => part.get(test) > *value,
                        Op::LessThan => part.get(test) < *value,
                    };
                    if test {
                        return ProcessResult::Workflow(workflow.clone());
                    }
                }
            }
        }
        panic!("No rule matched");
    }
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

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let s = s.trim();
        let s = s.strip_prefix('{').unwrap();
        let s = s.strip_suffix('}').unwrap();
        let s = s.trim();
        let mut parts = s.split(',');
        let x = parts.next().unwrap();
        let m = parts.next().unwrap();
        let a = parts.next().unwrap();
        let s = parts.next().unwrap();
        let x = x.split_once('=').unwrap().1.trim().parse().unwrap();
        let m = m.split_once('=').unwrap().1.trim().parse().unwrap();
        let a = a.split_once('=').unwrap().1.trim().parse().unwrap();
        let s = s.split_once('=').unwrap().1.trim().parse().unwrap();
        Self { x, m, a, s }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input/d19p1.txt").unwrap();

    let (workflows, parts) = input.split_once("\r\r\n\r\r").unwrap();

    let workflows = workflows
        .trim()
        .lines()
        .map(Workflow::from)
        .collect::<Vec<_>>();

    let out = parts
        .trim()
        .lines()
        .map(Part::from)
        .map(|part| {
            let outcome = process(&part, &workflows);
            match outcome {
                Outcome::Accept => part.sum(),
                Outcome::Reject => 0,
            }
        })
        .sum::<u32>();

    println!("{}", out);
}

fn process(part: &Part, workflows: &Vec<Workflow>) -> Outcome {
    let mut input = "in".to_string();
    loop {
        let workflow = workflows.iter().find(|w| w.name == input).unwrap();
        let result = workflow.process(part);
        match result {
            ProcessResult::Outcome(outcome) => return outcome,
            ProcessResult::Workflow(str) => input = str,
        }
    }
}
