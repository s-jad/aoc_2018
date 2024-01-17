use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn parse_nodes(data: &mut &[usize]) -> Node {
    let child_count = data[0];
    let metadata_count = data[1];
    *data = &data[2..];

    let mut children = Vec::new();
    for _ in 0..child_count {
        children.push(parse_nodes(data));
    }

    let metadata = data[..metadata_count].to_vec();
    *data = &data[metadata_count..];

    Node { children, metadata }
}

fn process(input: &str) -> usize {
    let nums = input
        .split_whitespace()
        .into_iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_vec();

    let tree = parse_nodes(&mut nums.as_slice());

    sum_metadata(&tree)
}

fn sum_metadata(n: &Node) -> usize {
    n.metadata.iter().sum::<usize>() + n.children.iter().map(sum_metadata).sum::<usize>()
}

fn main() {
    let input = include_str!("../../test.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
