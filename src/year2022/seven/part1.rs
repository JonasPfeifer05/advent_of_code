use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Deref;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::thread::current;
use crate::year2022::seven::part1::Node::{DIRECTORY, FILE};

#[derive(Debug)]
pub enum Node {
    FILE(u32),
    DIRECTORY(String, Vec<Box<Node>>, Option<String>),
}

impl Node  {
    pub fn set_parent(&mut self, parent: String) {
        match self {
            FILE(_) => {}
            DIRECTORY(_, _, ref mut prev) => {
                *prev = Some(parent);
            }
        }
    }

    pub fn get_parent(&self) -> Option<String> {
        match self {
            FILE(_) => {None}
            DIRECTORY(nameing, _, _) => {Some(nameing.clone())}
        }
    }

    pub fn get_name(&self) -> Option<String> {
        match self {
            FILE(_) => {None}
            DIRECTORY(name, _, _) => {Some(name.clone())}
        }
    }
}

pub fn find_rep(search_name: String, root: &mut Box<Node>) -> Option<&mut Box<Node>> {
    match root.as_mut() {
        FILE(_) => {
            None
        }
        DIRECTORY(ref name, ref mut content, _) => {
            for node in content {
                match node.as_mut() {
                    FILE(_) => {}
                    DIRECTORY(naming, _, _) => {
                        if *naming == search_name.clone() {
                            return Some(node);
                        }
                        let recursive = find_rep(search_name.clone(), node);
                        if recursive.is_some() { return recursive }
                    }
                }
            }
            None
        }
    }
}

pub fn add_node(dir: String, mut node: Node, filesystem: &mut Box<Node>) {
    let directory = if dir.clone() == "/" { filesystem } else { find_rep(dir.clone(), filesystem).unwrap() };
    node.set_parent(dir.clone());
    match directory.as_mut() {
        FILE(_) => {}
        DIRECTORY(_, ref mut content, _) => {
            content.push(Box::new(node));
        }
    }
}

pub fn get_dirs<'a, 'b>(root: &'a Box<Node>, list: &'b mut Vec<&'a Box<Node>>) where 'a: 'b {
    list.push(root);
    match root.as_ref() {
        FILE(_) => {}
        DIRECTORY(_, content, _) => {
            for node in content {
                match node.as_ref() {
                    FILE(_) => {}
                    DIRECTORY(_, _, _) => {
                        get_dirs(node, list);
                    }
                }
            }
        }
    }
}

pub fn size_of_dir(dir: &Node, size: &mut u32) {
    match dir {
        FILE(value) => {
            *size += value;
        }
        DIRECTORY(_, content, _) => {
            for node in content {
                size_of_dir(node, size);
            }
        }
    }
}

pub fn run() -> Result<(), String> {
    let file = File::open(Path::new("res/year2022/seven/test.txt")).map_err(|_| String::from("Could not read file!"))?;
    let mut lines = io::BufReader::new(file).lines();

    let mut filesystem = Box::new(DIRECTORY("/".to_string(), Vec::new(), None));

    let mut current = "/".to_string();

    for line in lines {
        println!("{:?}", line);
        if let Ok(line) = line {
            let tokens: Vec<_> = line.split(" ").collect();

            match tokens.get(0).unwrap() {
                &"$" => {
                    match tokens.get(1).unwrap() {
                        &"cd" => {
                            match tokens.get(2).unwrap() {
                                &"/" => {}
                                &".." => {
                                    current = find_rep(current.clone(), &mut filesystem).unwrap().get_parent().unwrap();
                                }
                                &_ => {
                                    current = tokens.get(2).unwrap().parse().unwrap();
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                &_ => {
                    match tokens.get(0).unwrap() {
                        &"dir" => {
                            add_node(current.clone(), DIRECTORY(tokens.get(1).unwrap().parse().unwrap(), Vec::new(), None), &mut filesystem);
                        }
                        &_ => {
                            add_node(current.clone(), FILE(tokens.get(0).unwrap().parse().unwrap()), &mut filesystem);
                        }
                    }
                }
            }
        }
    }

    let mut vec = Vec::new();
    get_dirs(&filesystem, &mut vec);
    println!("{:?}", vec);

    let mut sizes = Vec::new();
    let mut size = 0;
    for dir in vec {
        size_of_dir(dir, &mut size);
        sizes.push(size);
        size = 0;
    }

    let sum: u32 = sizes.iter().filter(|x| **x < 100000).sum();

    println!("{:?}", sum);

    Ok(())
}