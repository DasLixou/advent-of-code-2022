use std::rc::Rc;

use hashbrown::HashMap;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Directory(Option<Rc<Directory>>, String);

impl Directory {
    #[inline]
    pub fn root() -> Rc<Self> {
        Rc::new(Self(None, "".into()))
    }

    #[inline]
    pub fn enter(dir: Rc<Directory>, name: impl Into<String>) -> Rc<Self> {
        Rc::new(Self(Some(dir), name.into()))
    }

    #[inline]
    pub fn leave(&self) -> Rc<Self> {
        self.0
            .as_ref()
            .expect("Can't go any higher in file-system")
            .to_owned()
    }

    #[inline]
    pub fn has_parent(&self) -> bool {
        self.0.is_some()
    }
}

pub fn normal() {
    let input = include_str!("input.txt");
    let mut directories: HashMap<Rc<Directory>, usize> = HashMap::new(); // flat representation of the given file-system with the size of the directory
    let mut current_dir = Directory::root();

    input.lines().for_each(|line| match line {
        cmd if cmd.starts_with('$') => match &cmd[2..] {
            ls if ls.starts_with("ls") => {}
            cd if cd.starts_with("cd") => match &cd[3..] {
                root if root.eq("/") => {
                    current_dir = Directory::root();
                }
                leave if leave.eq("..") => {
                    current_dir = current_dir.leave();
                }
                enter => {
                    current_dir = Directory::enter(current_dir.clone(), enter);
                }
            },
            _ => panic!("Unknown command"),
        },
        dir if dir.starts_with("dir") => {}
        file => {
            let info = file.split(' ').collect::<Vec<&str>>();
            let size = info[0].parse::<usize>().unwrap();
            let mut dir = current_dir.clone();
            loop {
                if !directories.contains_key(&dir) {
                    directories.insert(dir.to_owned(), 0);
                }
                *directories.get_mut(&dir).unwrap() += size;
                if !dir.has_parent() {
                    break;
                }
                dir = dir.leave();
            }
        }
    });

    let result = directories
        .values()
        .map(|size| *size)
        .filter(|size| *size <= 100000usize)
        .sum::<usize>();
    println!("Result: {result}");
}

pub fn bonus() {
    let input = include_str!("input.txt");
    let mut directories: HashMap<Rc<Directory>, usize> = HashMap::new(); // flat representation of the given file-system with the size of the directory
    let mut current_dir = Directory::root();

    input.lines().for_each(|line| match line {
        cmd if cmd.starts_with('$') => match &cmd[2..] {
            ls if ls.starts_with("ls") => {}
            cd if cd.starts_with("cd") => match &cd[3..] {
                root if root.eq("/") => {
                    current_dir = Directory::root();
                }
                leave if leave.eq("..") => {
                    current_dir = current_dir.leave();
                }
                enter => {
                    current_dir = Directory::enter(current_dir.clone(), enter);
                }
            },
            _ => panic!("Unknown command"),
        },
        dir if dir.starts_with("dir") => {}
        file => {
            let info = file.split(' ').collect::<Vec<&str>>();
            let size = info[0].parse::<usize>().unwrap();
            let mut dir = current_dir.clone();
            loop {
                if !directories.contains_key(&dir) {
                    directories.insert(dir.to_owned(), 0);
                }
                *directories.get_mut(&dir).unwrap() += size;
                if !dir.has_parent() {
                    break;
                }
                dir = dir.leave();
            }
        }
    });

    const MAX_DISC_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;
    let used_space = directories.get(&Directory::root()).unwrap().to_owned();
    let space_to_delete = REQUIRED_SPACE - (MAX_DISC_SPACE - used_space);

    let result = directories
        .values()
        .map(|size| *size)
        .filter(|size| *size >= space_to_delete)
        .min()
        .unwrap();
    println!("Result: {result}");
}
