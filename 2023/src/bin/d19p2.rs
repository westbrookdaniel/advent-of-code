use std::ops::RangeInclusive;

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

#[derive(Debug, Clone)]
struct RangePart {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

impl RangePart {
    fn get(&self, test: String) -> &RangeInclusive<u32> {
        match test.as_str() {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => panic!("unknown test"),
        }
    }

    fn set(&mut self, test: String, range: RangeInclusive<u32>) {
        match test.as_str() {
            "x" => self.x = range,
            "m" => self.m = range,
            "a" => self.a = range,
            "s" => self.s = range,
            _ => panic!("unknown test"),
        }
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

    let range_part = RangePart {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    // work from ends to find n of parts that lead to an Outcome::Accept
    let parts = process(&workflows, range_part, "in");

    println!("{:?}", parts);
    // 167409079868000
    //
    // 1176306889923954 first
    // 255744095984001 merging parts
    // 26676912540141 upate merging?
}

// recursive fn that splits where condition is found and returns a vector of range parts
fn process(workfows: &Vec<Workflow>, range_part: RangePart, workflow_name: &str) -> Vec<RangePart> {
    let workflow = workfows.iter().find(|w| w.name == workflow_name).unwrap();

    // let extras = vec![];
    // let mut curr_part = range_part;

    // for rule in workflow.rules {
    //     let processed = process_rule(rule, range_part);

    //     todo!()
    // }

    todo!()
}

enum ProcessedRule {
    SplitAccept(RangePart, RangePart),
    SplitReject(RangePart, RangePart),
    SplitTo(RangePart, RangePart, String),
    To(RangePart, String),
    Accept(RangePart),
    Reject(RangePart),
}

fn process_rule(rule: Rule, range_part: RangePart) -> ProcessedRule {
    match rule {
        Rule::Outcome(Outcome::Accept) => ProcessedRule::Accept(range_part),
        Rule::Outcome(Outcome::Reject) => ProcessedRule::Reject(range_part),
        Rule::Workflow(name) => ProcessedRule::To(range_part, name),
        Rule::ConditionToOutcome {
            outcome,
            test,
            op,
            value,
        } => {
            let (cur, extra) = split_range(range_part, test, op, value);

            if let Some(extra) = extra {
                if let Some(cur) = cur {
                    match outcome {
                        Outcome::Accept => ProcessedRule::SplitAccept(cur, extra),
                        Outcome::Reject => ProcessedRule::SplitReject(cur, extra),
                    }
                } else {
                    match outcome {
                        Outcome::Accept => ProcessedRule::Accept(extra),
                        Outcome::Reject => ProcessedRule::Reject(extra),
                    }
                }
            } else {
                if let Some(cur) = cur {
                    match outcome {
                        Outcome::Accept => ProcessedRule::Accept(cur),
                        Outcome::Reject => ProcessedRule::Reject(cur),
                    }
                } else {
                    panic!("no extra or cur");
                }
            }
        }
        Rule::ConditionToWorkflow {
            workflow,
            test,
            op,
            value,
        } => {
            let (cur, extra) = split_range(range_part, test, op, value);

            if let Some(extra) = extra {
                if let Some(cur) = cur {
                    ProcessedRule::SplitTo(cur, extra, workflow)
                } else {
                    ProcessedRule::To(extra, workflow)
                }
            } else {
                if let Some(cur) = cur {
                    ProcessedRule::To(cur, workflow)
                } else {
                    panic!("no extra or cur");
                }
            }
        }
    }
}

fn split_range(
    range_part: RangePart,
    test: String,
    op: Op,
    value: u32,
) -> (Option<RangePart>, Option<RangePart>) {
    let (truthy, falsey) = {
        // split range_part.get(test) into truthy range and falsey range
        // if one of the ranges is empty then return None for that range
        todo!()
    };

    (truthy, falsey)
}
