use std::{collections::HashMap, fs};

#[derive(Debug)]
struct FileSystem {
    dir: HashMap<String, Directory>,
    pwd: String,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    dir_name: String,
    sub_dir: Vec<String>,
    files: Vec<File>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            dir: HashMap::new(),
            pwd: "/".to_string(),
        };

        let root_dir = Directory::new("/", "/");

        fs.dir.insert("/".to_string(), root_dir);

        return fs;
    }

    fn cd(&mut self, dir: &str) {
        let pwd = self
            .dir
            .get(&String::from(self.pwd.clone()))
            .expect("dir non existent");

        if dir == ".." {
            self.pwd = pwd.dir_name.clone();
        } else if dir != "/" {
            self.pwd = self.pwd.clone() + dir + "/";
        }
    }

    fn mkdir(&mut self, name: &str) {
        let path = self.pwd.clone() + name + "/";

        self.dir
            .insert(path.clone(), Directory::new(name, &self.pwd));

        let pwd = self
            .dir
            .get_mut(&String::from(self.pwd.clone()))
            .expect("dir non existent");

        pwd.add_sub_dir(&path);
    }

    fn touch(&mut self, filename: &str, size: u32) {
        let pwd = self
            .dir
            .get_mut(&String::from(self.pwd.clone()))
            .expect("dir non existent");

        pwd.add_file(filename, size);
    }

    fn calculate_size(&self) {
        let mut total_sum: u32 = 0;

        for (dirname, directory) in self.dir.clone().into_iter() {
            // Files size
            let size = directory.get_size(self);

            if size < 100_000 {
                total_sum += size;
            }

            println!("{dirname} - {size}");
        }

        println!("{total_sum}");
    }
}

impl Directory {
    fn new(name: &str, pwd: &str) -> Directory {
        Directory {
            name: name.to_string(),
            dir_name: pwd.to_string(),
            sub_dir: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add_file(&mut self, filename: &str, size: u32) {
        self.files.push(File {
            name: filename.to_string(),
            size,
        })
    }

    fn add_sub_dir(&mut self, path: &str) {
        self.sub_dir.push(path.to_string());
    }

    fn get_size(&self, fs: &FileSystem) -> u32 {
        // Files size
        let mut total: u32 = 0;

        for file in self.files.clone() {
            total += file.size;
        }

        // Sub dir size
        for dir in self.sub_dir.clone() {
            println!("{dir}");

            let fs_dir = fs.dir.get(&dir).expect("404");

            let subdir_size = fs_dir.get_size(fs);

            total += subdir_size;
        }

        return total;
    }
}

fn main() {
    let mut file_system: FileSystem = FileSystem::new();

    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Get input lines
    let lines = input.lines().into_iter();

    for command in lines {
        execute_command(&mut file_system, command);
    }

    file_system.calculate_size();
}

fn execute_command(fs: &mut FileSystem, cmd: &str) {
    let split_line: Vec<&str> = cmd.split_whitespace().collect();

    if split_line[1] == "ls" {
        return;
    }

    if split_line[1] == "cd" {
        fs.cd(split_line[2]);
        return;
    }

    if split_line[0] == "dir" {
        fs.mkdir(split_line[1]);
        return;
    }

    fs.touch(split_line[1], split_line[0].parse().unwrap())
}
