use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use slab_tree::NodeRef;
use slab_tree::Tree;
use slab_tree::TreeBuilder;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

struct DirEntry {
    name: String,
    file_size: u64,
}

impl DirEntry {
    fn directory(name: &str) -> DirEntry {
        DirEntry {
            name: name.to_owned(),
            file_size: 0,
        }
    }

    fn file(name: &str, size: u64) -> DirEntry {
        DirEntry {
            name: name.to_owned(),
            file_size: size,
        }
    }

    fn is_dir(&self) -> bool {
        self.file_size == 0
    }
}

fn parse_commands(input: &Vec<String>) -> Tree<DirEntry> {
    let mut root = TreeBuilder::new()
        .with_root(DirEntry::directory("/"))
        .build();
    let mut cwd = root.root().unwrap().node_id();

    for line in input.into_iter().skip(1) {
        // assume input starts with cd /
        if line.starts_with("$") {
            let parts: Vec<&str> = line.split(" ").collect();
            let command = parts[1];
            match command {
                "cd" => {
                    // change current working directory
                    let arg = parts[2];
                    if arg == ".." {
                        let parent_id = root.get(cwd).unwrap().parent().unwrap().node_id();
                        cwd = parent_id;
                    } else {
                        let cwd_entry = root.get(cwd).unwrap();
                        let child = cwd_entry.children().find(|c| c.data().name == arg).unwrap();
                        cwd = child.node_id();
                    }
                }
                "ls" => {}
                _ => panic!("Unhandled command"),
            }
        } else if line.starts_with("dir") {
            let name = line.split(" ").nth(1).unwrap();

            let mut node = root.get_mut(cwd).unwrap();
            node.append(DirEntry::directory(&name));
        } else {
            // file
            let split = line.split(" ").collect::<Vec<&str>>();
            let file_size = split[0].parse::<u64>().unwrap();
            let file_name = split[1];

            let mut node = root.get_mut(cwd).unwrap();
            node.append(DirEntry::file(file_name, file_size));
        }
    }
    root
}

fn sum_file_size(tree: &NodeRef<DirEntry>) -> u64 {
    tree.traverse_pre_order()
        .fold(0, |accum, node| accum + node.data().file_size)
}

fn sum_size_with_limit(tree: NodeRef<DirEntry>, limit: Option<u64>) -> u64 {
    let size_limit = match limit {
        Some(n) => n,
        None => u64::MAX,
    };
    tree.traverse_level_order().fold(0, |accum, node| {
        if node.data().is_dir() {
            let size = sum_file_size(&node);
            if size < size_limit {
                return accum + size;
            }
        }
        accum
    })
}

#[aoc(day7, part1)]
pub fn sum_directories_smaller_than_100k(input: &Vec<String>) -> u64 {
    let dir = parse_commands(input);
    sum_size_with_limit(dir.root().unwrap(), Some(100_000))
}

fn smallest_bigger_than(tree: &NodeRef<DirEntry>, minimum_size: u64) -> Option<u64> {
    let mut best_size = u64::MAX;

    tree.traverse_level_order().skip(1).for_each(|node| {
        if node.data().is_dir() {
            let size = sum_file_size(&node);
            if size >= minimum_size && size < best_size {
                best_size = size;
            }
        }
    });

    if best_size != u64::MAX {
        return Some(best_size);
    }

    None
}

#[aoc(day7, part2)]
pub fn find_directory_free_30gb(input: &Vec<String>) -> u64 {
    let dir = parse_commands(input);
    let total = sum_file_size(&dir.root().unwrap());
    let free = 70_000_000 - total;
    let missing = 30_000_000 - free;

    smallest_bigger_than(&dir.root().unwrap(), missing).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY07_EXAMPLE: &str = "$ cd /
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
    fn test_day7_example() {
        let input = parse_input(DAY07_EXAMPLE);
        assert_eq!(sum_directories_smaller_than_100k(&input), 95_437);
    }

    #[test]
    fn test_day7_example2() {
        let input = parse_input(DAY07_EXAMPLE);
        assert_eq!(find_directory_free_30gb(&input), 24_933_642);
    }
}
