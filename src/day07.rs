use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, PartialEq)]
enum Command {
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match &s[..2] {
            "ls" => Ok(Command::Ls),
            "cd" => Ok(Command::Cd(s[2..].trim().to_owned())),
            _ => Err(anyhow!("unreachable")),
        }
    }
}

#[derive(Default, Debug)]
pub struct Folder {
    id: usize,
    name: String,
    size: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Folder {
    pub fn new(id: usize, name: String, size: usize, parent: Option<usize>) -> Self {
        Folder {
            id,
            name: name.trim().into(),
            size,
            parent,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct DirectoryTree {
    arena: Vec<Folder>,
}

impl DirectoryTree {
    fn folder(&mut self, name: String, parent: Option<usize>) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.name == name {
                match (parent, node.parent) {
                    (Some(p1), Some(p2)) => {
                        if p1 == p2 {
                            return node.id;
                        }
                    }
                    (None, None) => return node.id,
                    _ => (),
                }
            }
        }

        // Otherwise, add new node
        let id = self.arena.len();
        self.arena.push(Folder::new(id, name, 0, parent));
        id
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        if !self.arena[parent].children.contains(&child) {
            self.arena[parent].children.push(child)
        }
        self.arena[child].parent = Some(parent)
    }

    fn size(&self, id: usize) -> usize {
        let node = &self.arena[id];
        let mut size = node.size;
        for child in &node.children {
            size += self.size(*child)
        }
        size
    }
}

pub fn parse_input(input: &str) -> Result<DirectoryTree> {
    let mut directory = DirectoryTree::default();
    let mut current_folder = directory.folder("/".into(), None);
    // first line is always cd / to set the root, skip it
    for block in input.split("\n$").skip(1) {
        // first line is a command
        let mut lines = block.lines();
        let command = match lines.next() {
            Some(command) => command.trim().parse::<Command>()?,
            None => continue,
        };
        // println!("{:?}", command);
        match command {
            Command::Cd(s) => {
                if s == ".." {
                    // move up
                    current_folder = match directory.arena[current_folder].parent {
                        Some(c) => c,
                        None => Err(anyhow!("Top reach too soon with {:#?}", directory))?,
                    }
                } else {
                    // move down
                    let folder = directory.folder(s, Some(current_folder));
                    directory.add_child(current_folder, folder);
                    current_folder = folder;
                }
            }
            Command::Ls => {
                // println!("In LS");
                for line in lines {
                    // println!("{}", line);
                    if line.starts_with("dir") {
                        // add a subdir
                        let child = directory.folder(line[3..].to_string(), Some(current_folder));
                        directory.add_child(current_folder, child);
                    } else {
                        let (n, _) = line.split_once(" ").unwrap();
                        directory.arena[current_folder].size += n.parse::<usize>().unwrap();
                    }
                }
            }
        }
        // println!("current folder: {:?}", current_folder);
    }

    Ok(directory)
}

pub fn part1(tree: DirectoryTree) -> usize {
    (0..tree.arena.len())
        .map(|dir| tree.size(dir))
        .filter(|size| size <= &100000)
        .sum()
}

pub fn part2(tree: DirectoryTree) -> usize {
    const SIZE_TO_FREE: usize = 70000000;
    // 0 is the root
    let unused_space = SIZE_TO_FREE - tree.size(0);
    (0..tree.arena.len())
        .map(|dir| tree.size(dir))
        .filter(|&size| unused_space + size > 30000000)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse_input() -> Result<()> {
        let dir = match parse_input(INPUT) {
            Ok(d) => d,
            Err(_) => unreachable!(),
        };

        // 4 directories: /, a, e, d
        assert_eq!(&dir.arena.len(), &4);
        let root = &dir.arena[0];
        assert_eq!(root.name, "/");
        // size is only the files directly in the folder
        assert_eq!(root.size, 23352670);
        // '/' has 2 children
        assert_eq!(root.children.len(), 2);
        let a = &dir.arena[1];
        assert_eq!(a.size, 94269);
        // total sizes
        assert_eq!(dir.size(0), 48381165); // '/4
        assert_eq!(dir.size(1), 94853); // 'a'
        assert_eq!(dir.size(2), 24933642); // 'd'
        assert_eq!(dir.size(3), 584); // 'e'
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let dir = parse_input(INPUT)?;
        assert_eq!(part1(dir), 95437);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let dir = parse_input(INPUT)?;
        assert_eq!(part2(dir), 24933642);
        Ok(())
    }
}
