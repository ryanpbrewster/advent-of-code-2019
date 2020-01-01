extern crate files;

use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[test]
fn part1_smoke() {
    let input = r#"
        A)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
    "#;
    let deps: Vec<Dependency> = input
        .lines()
        .map(|ln| ln.trim())
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.parse())
        .collect::<Result<_, _>>()
        .expect("parse input");
    assert_eq!(sum_depths(deps), 42);
}

#[test]
fn part1() {
    let input = files::read!("input.txt");
    let deps: Vec<Dependency> = input
        .lines()
        .map(|ln| ln.trim())
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.parse())
        .collect::<Result<_, _>>()
        .expect("parse input");
    assert_eq!(sum_depths(deps), 139597);
}

#[test]
fn part2_smoke() {
    let input = r#"
        A)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
    "#;
    let deps: Vec<Dependency> = input
        .lines()
        .map(|ln| ln.trim())
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.parse())
        .collect::<Result<_, _>>()
        .expect("parse input");
    assert_eq!(distance(deps, "L", "I"), Some(5));
}

#[test]
fn part2() {
    let input = files::read!("input.txt");
    let deps: Vec<Dependency> = input
        .lines()
        .map(|ln| ln.trim())
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.parse())
        .collect::<Result<_, _>>()
        .expect("parse input");
    assert_eq!(distance(deps, "YOU", "SAN"), Some(286 + 2));
}

struct Dependency {
    before: String,
    after: String,
}

impl FromStr for Dependency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(')');
        let before = it.next().ok_or(())?.to_owned();
        let after = it.next().ok_or(())?.to_owned();
        Ok(Dependency { before, after })
    }
}

fn sum_depths(deps: Vec<Dependency>) -> usize {
    let mut direct_prereq_count: HashMap<String, usize> = {
        let mut builder = HashMap::new();
        for d in &deps {
            builder.entry(d.before.clone()).or_default();
            *builder.entry(d.after.clone()).or_default() += 1;
        }
        builder
    };
    let mut direct_children: HashMap<String, Vec<String>> = {
        let mut builder: HashMap<String, Vec<String>> = HashMap::new();
        for d in &deps {
            builder
                .entry(d.before.clone())
                .or_default()
                .push(d.after.clone());
        }
        builder
    };

    struct Item {
        depth: usize,
        value: String,
    }

    let mut q = VecDeque::new();
    for (v, &cnt) in direct_prereq_count.iter() {
        if cnt == 0 {
            q.push_back(Item {
                depth: 0,
                value: v.clone(),
            });
        }
    }

    let mut total = 0;
    while let Some(Item { depth, value }) = q.pop_front() {
        total += depth;
        for child in direct_children.remove(&value).unwrap_or_default() {
            let cnt = direct_prereq_count.get_mut(&child).unwrap();
            *cnt -= 1;
            if *cnt == 0 {
                q.push_back(Item {
                    value: child,
                    depth: depth + 1,
                });
            }
        }
    }

    total
}

fn distance(deps: Vec<Dependency>, src: &str, dst: &str) -> Option<usize> {
    let neighbors: HashMap<String, Vec<String>> = {
        let mut builder: HashMap<String, Vec<String>> = HashMap::new();
        for d in deps {
            builder
                .entry(d.before.clone())
                .or_default()
                .push(d.after.clone());
            builder
                .entry(d.after.clone())
                .or_default()
                .push(d.before.clone());
        }
        builder
    };

    struct Item {
        depth: usize,
        value: String,
    }

    let mut vis = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(Item {
        depth: 0,
        value: src.to_owned(),
    });
    while let Some(Item { depth, value }) = q.pop_front() {
        if value == dst {
            return Some(depth);
        }
        if !vis.insert(value.clone()) {
            continue;
        }
        for child in &neighbors[&value] {
            q.push_back(Item {
                depth: depth + 1,
                value: child.clone(),
            });
        }
    }
    None
}
