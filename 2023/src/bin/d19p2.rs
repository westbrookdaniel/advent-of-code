use std::ops::Range;

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
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl RangePart {
    fn get(&self, s: &String) -> Range<u32> {
        let s = s.trim();
        match s {
            "x" => self.x.clone(),
            "m" => self.m.clone(),
            "a" => self.a.clone(),
            "s" => self.s.clone(),
            _ => panic!("Unknown part"),
        }
    }

    fn set(&mut self, s: &String, r: Range<u32>) {
        let s = s.trim();
        match s {
            "x" => self.x = r,
            "m" => self.m = r,
            "a" => self.a = r,
            "s" => self.s = r,
            _ => panic!("Unknown part"),
        }
    }

    fn total_possible(&self) -> u64 {
        let x = self.x.end - self.x.start;
        let m = self.m.end - self.m.start;
        let a = self.a.end - self.a.start;
        let s = self.s.end - self.s.start;

        x as u64 * m as u64 * a as u64 * s as u64
    }

    fn merge(&self, other: &RangePart) -> RangePart {
        RangePart {
            x: self.x.start.min(other.x.start)..self.x.end.max(other.x.end),
            m: self.m.start.min(other.m.start)..self.m.end.max(other.m.end),
            a: self.a.start.min(other.a.start)..self.a.end.max(other.a.end),
            s: self.s.start.min(other.s.start)..self.s.end.max(other.s.end),
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
    let parts = ends
        .iter()
        .map(|end| range_for_part(&workflows, *end))
        .collect::<Vec<_>>();

    for o in &parts {
        println!("{:?}", o);
    }
    println!();

    let mut part = parts[0].clone();
    for p in parts {
        part = part.merge(&p);
    }

    println!("{:?}", part);

    let n = part.total_possible();

    let total = RangePart {
        x: 1..4000,
        m: 1..4000,
        a: 1..4000,
        s: 1..4000,
    }
    .total_possible();

    let n = total - n;

    // let _ = n_of_part(&workflows, ends[1]);

    println!("{}", n);
    // 167409079868000
    //
    // 1176306889923954 first
    // 255744095984001 merging parts
}

// recursive find n of parts with ranges
fn range_for_part(workflows: &Vec<Workflow>, end: (usize, usize)) -> RangePart {
    // println!("{:?}", end);

    let out = get_range_for_rule(
        workflows,
        end.0,
        end.1,
        RangePart {
            x: 1..4000,
            m: 1..4000,
            a: 1..4000,
            s: 1..4000,
        },
    );

    if out.len() > 1 {
        println!();
        println!("OUT");
        for o in &out {
            println!("{:?}", o);
        }
        panic!("More than one range?!");
    }

    let range_part = &out[0];

    range_part.clone()
}

fn get_range_for_rule(
    workflows: &Vec<Workflow>,
    workflow_i: usize,
    rule_i: usize,
    range_part: RangePart,
) -> Vec<RangePart> {
    let mut i = rule_i as i32;
    let mut range_part = range_part;
    let mut other = vec![];
    let workflow = &workflows[workflow_i];

    while i >= -1 {
        if i == -1 {
            let n = workflow.name.clone();
            // println!("range_part {:?}", range_part);
            // println!("other {:?}", other);
            // println!("TO {:?}", n);
            if n == "in" {
                break;
            }

            let rules = find_rules_for_workflow(workflows, &n);
            let mut r = rules.iter().fold(vec![], |mut acc, rule| {
                let mut r = get_range_for_rule(workflows, rule.0, rule.1, range_part.clone());
                acc.append(&mut r);
                acc
            });

            if other.len() > 0 {
                range_part = other.pop().unwrap();
                other.append(&mut r);
            }

            break;
        }

        let rule = &workflow.rules[i as usize];

        match rule {
            Rule::Outcome(Outcome::Accept) => {
                i -= 1;
            }
            Rule::Workflow(to_w) => {
                let rules = find_rules_for_workflow(workflows, &to_w);
                let mut r = rules.iter().fold(vec![], |mut acc, rule| {
                    let mut r = get_range_for_rule(workflows, rule.0, rule.1, range_part.clone());
                    acc.append(&mut r);
                    acc
                });

                other.append(&mut r);
                break;
            }
            Rule::ConditionToOutcome {
                outcome,
                test,
                op,
                value,
            } => {
                let upper = range_part.get(test).end;
                let lower = range_part.get(test).start;

                let (l, r) = match op {
                    Op::GreaterThan => (lower..*value, *value + 1..upper),
                    Op::LessThan => (*value + 1..upper, lower..*value),
                };

                let curr = if i == rule_i as i32 {
                    match outcome {
                        Outcome::Accept => l,
                        Outcome::Reject => r,
                    }
                } else {
                    match outcome {
                        Outcome::Accept => r,
                        Outcome::Reject => l,
                    }
                };

                i -= 1;
                range_part.set(test, curr);
            }
            Rule::ConditionToWorkflow {
                workflow: _,
                test,
                op,
                value,
            } => {
                let upper = range_part.get(test).end;
                let lower = range_part.get(test).start;

                let (n, curr) = match op {
                    Op::LessThan => (lower..*value, *value + 1..upper),
                    Op::GreaterThan => (*value + 1..upper, lower..*value),
                };

                let mut n_range_part = range_part.clone();
                n_range_part.set(test, n);

                i -= 1;
                range_part.set(test, curr);
            }
            r => panic!("Unexpected rule {:?}", r),
        };
    }

    [vec![range_part], other].concat()
}

fn name_matches(rule: &Rule, name: &String) -> bool {
    match rule {
        Rule::Outcome(_) => false,
        Rule::Workflow(w) => w == name,
        Rule::ConditionToOutcome { outcome, .. } => match outcome {
            Outcome::Accept => false,
            Outcome::Reject => false,
        },
        Rule::ConditionToWorkflow { workflow, .. } => workflow == name,
    }
}

fn find_rules_for_workflow(workflows: &Vec<Workflow>, name: &String) -> Vec<(usize, usize)> {
    let mut rules = vec![];
    for (i, workflow) in workflows.iter().enumerate() {
        workflow
            .rules
            .iter()
            .filter(|r| name_matches(r, name))
            .enumerate()
            .for_each(|(j, _)| rules.push((i, j)));
    }
    rules
}
