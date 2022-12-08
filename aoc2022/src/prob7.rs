use std::cmp::min;
use crate::prob7::Node::{Directory, File};
use itertools::Itertools;
use std::fmt::format;
use std::str::FromStr;

#[derive(Debug)]
enum Node {
    Directory(String, Vec<Node>),
    File(String, usize),
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Directory(n, _) => n.as_str(),
            Node::File(n, _) => n.as_str(),
        }
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            Ok(Directory(s["dir ".len()..].to_string(), Vec::new()))
        } else {
            let mut size_and_name = s.split(" ");
            let size = size_and_name
                .next()
                .unwrap()
                .parse()
                .expect(&format!("Unable to parse size: {}", s));
            Ok(File(size_and_name.next().expect("").to_string(), size))
        }
    }
}

fn read_directory_tree<'a, I>(lines: &mut I, current_dir: &mut Node)
where
    I: Iterator<Item = &'a str> + Clone,
{
    if let Node::Directory(name, children) = current_dir {
        while let Some(line) = lines.next() {
            if line.starts_with("$ cd ..") {
                return;
            } else if line.starts_with("$ cd") {
                let to_dir = &line["$ cd ".len()..];
                let dir = children
                    .iter_mut()
                    .find(|c| c.name() == to_dir)
                    .expect(&format!("Couldn't find directory {}", to_dir));
                read_directory_tree(lines, dir);
            } else if line.starts_with("$ ls") {
                let mut c: Vec<_> = lines
                    .clone()
                    .take_while(|l| !l.starts_with("$"))
                    .map(|line| line.parse::<Node>().unwrap())
                    .collect();

                children.append(&mut c);
            }
        }
    } else {
        panic!("Not a directory");
    }
}

fn size(node: &Node) -> usize {
    match node {
        Directory(_, children) => children.iter().map(|c| size(c)).sum(),
        File(_, size) => *size,
    }
}

fn find_nodes_with_size(node: &Node) -> usize {
    match node {
        Directory(_, children) => {
            let size = size(node);
            let children_size = children.iter().map(|c| find_nodes_with_size(c)).sum::<usize>();
            if size <= 100000 {
                size + children_size
            } else {
                children_size
            }
        }
        File(_, _) => 0
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut root = Directory("/".to_string(), Vec::new());
    read_directory_tree(&mut input.lines().skip(1), &mut root);

    //println!("{:?}", root);
    find_nodes_with_size(&root)

}

pub fn solve_part_2(input: &str) -> usize {
    let mut root = Directory("/".to_string(), Vec::new());
    read_directory_tree(&mut input.lines().skip(1), &mut root);


    let used_space = size(&root);
    let additional_free_space_required = 30000000 - (70000000 - used_space);
    find_smallest_dir_with_size(&root, additional_free_space_required, usize::MAX)
}


fn find_smallest_dir_with_size(node: &Node, at_least: usize, mut smallest_dir_so_far: usize) -> usize {
    match node {
        Directory(_, children) => {
            let size = size(node);
            if size >= at_least {
                smallest_dir_so_far = min(smallest_dir_so_far, size);
                let children_smallest = children.iter()
                    .map(|c| find_smallest_dir_with_size(c, at_least, smallest_dir_so_far)).min().unwrap();

                min(smallest_dir_so_far, children_smallest)
            } else {
                usize::MAX
            }
        }
        File(_, _) => usize::MAX
    }
}