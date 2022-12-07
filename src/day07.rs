use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use slab_tree::Tree;
use slab_tree::TreeBuilder;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

struct DirEntry {
    name : String,
    file_size : u64,
}

impl DirEntry {
    fn new(name: &str, size: u64) -> DirEntry {
        DirEntry { name: name.to_owned(), file_size: size }        
    }
}

fn parse_commands(input: &Vec<String>) -> Tree<DirEntry> {
    let mut root = TreeBuilder::new().with_root(DirEntry::new("/", 0)).build();
    let mut cwd = root.root().unwrap().node_id();

    for command in input.into_iter().skip(1) {
        if command.starts_with("$ cd ") {
            let path = command.replace("$ cd ", "");
            if path == ".." {
                let cwd2 =root.get_mut(cwd).unwrap().parent().unwrap().node_id();
                cwd = cwd2;
            }
            else {
                for dir in root.get(cwd).unwrap().children() {
                    if dir.data().name == path {
                        let cwd2 = dir;
                        cwd = cwd2.node_id();
                    }
                }
            }
        }
        else if command.starts_with("$ ls") {

        }
        else if command.starts_with("dir") { // create a dir
            let name = command.replace("dir ", "");

            let mut d = root.get_mut(cwd).unwrap();
            d.append(DirEntry::new(&name, 0));

        }
        else { // file
            let split : Vec<&str> = command.split(' ').collect();
            let _size = split.iter().nth(0).unwrap().parse::<u64>().unwrap();
            let _name = split.iter().nth(1).unwrap();

            let mut d = root.get_mut(cwd).unwrap();
            d.append(DirEntry::new(&_name, _size));
        }
    }
    root
}

fn sum_file_size(tree: slab_tree::NodeRef<DirEntry>) -> u64 {
    let mut sum = 0;
    for entry in tree.children() {
        let name : String = String::from(&entry.data().name);
        let size = entry.data().file_size;
        if size > 0 {
            println!("file.add {} from folder {} {}", name, tree.data().name, size);
            sum += size
        }
        else {
            sum += sum_file_size(entry);
        }
    }
    sum
}

fn sum_size(tree: slab_tree::NodeRef<DirEntry>) -> u64 {
    let mut sum = tree.data().file_size;
    for entry in tree.children() {
        let name : String = String::from(&entry.data().name);
        let size = entry.data().file_size;
        if size == 0 { // dir
            let files_size = sum_file_size(entry);
            if files_size < 100000 {
                println!("dir.add folder {} {}", name, files_size);
                sum += files_size;
            }
        }
    }
    for entry in tree.children() {
        if entry.data().file_size == 0 {
            sum += sum_size(entry)
        }
    }
    sum
}

#[aoc(day7, part1)]
pub fn sum_directories_smaller_than_100k(input: &Vec<String>) -> u64 {
    const limit : u64 = 100000; // preparing for this to be a parameter
    let dir = parse_commands(input);
    sum_size(dir.root().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY07_EXAMPLE : &str = "$ cd /
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
        assert_eq!(sum_directories_smaller_than_100k(&input), 95437);
    }

}
