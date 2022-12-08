use std::fs;

type Matrix<T> = Vec<Vec<T>>;

struct Forest {
    trees: Matrix<u32>,
}

impl Forest {
    fn new(trees: Matrix<u32>) -> Forest {
        Forest { trees }
    }

    fn get_visible_trees(&self) -> u32 {
        // Get edges
        let x_trees = self.trees[0].len() as u32;
        let y_trees = self.trees.len() as u32;
        let mut total_visible_trees = x_trees * 2 + y_trees * 2 - 4;

        for tree_x in 1..(x_trees - 1) {
            for tree_y in 1..(y_trees - 1) {
                let is_visible = self.is_tree_visible(tree_x as usize, tree_y as usize);

                if is_visible {
                    total_visible_trees += 1;
                }
            }
        }

        return total_visible_trees;
    }

    fn is_tree_visible(&self, tree_x: usize, tree_y: usize) -> bool {
        // Initialize visibility
        // Left - Right - Top - Bottom
        let mut visibility = [true; 4];

        // Get tree height
        let height = self.trees[tree_y][tree_x];

        // Check row visibility
        let tree_row = self.trees[tree_y].clone();

        for x in 0..tree_x {
            if height <= tree_row[x] {
                visibility[0] = false;
                break;
            }
        }

        if visibility[0] == true {
            println!("Left Visible Tree {tree_x} - {tree_y} ({height})");

            return true;
        }

        for x in (tree_x + 1)..tree_row.len() {
            if height <= tree_row[x] {
                visibility[1] = false;
                break;
            }
        }

        if visibility[1] == true {
            println!("Right Visible Tree {tree_x} - {tree_y} ({height})");

            return true;
        }

        // Check column visibility
        // Build Column
        let mut column: Vec<u32> = Vec::new();

        for row in &self.trees {
            column.push(row[tree_x]);
        }

        // Top
        for y in 0..tree_y {
            if height <= column[y] {
                visibility[2] = false;
                break;
            }
        }

        if visibility[2] == true {
            println!("Top Visible Tree {tree_x} - {tree_y} ({height})");

            return true;
        }

        // Bottom

        for y in (tree_y + 1)..column.len() {
            if height <= column[y] {
                visibility[3] = false;
                break;
            }
        }

        if visibility[3] == true {
            println!("Bottom Visible Tree {tree_x} - {tree_y} ({height})");

            return true;
        }

        return false;
    }
}

fn main() {
    // Read input
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let matrix = get_height_matrix(input);

    let forest = Forest::new(matrix);

    let visible_trees = forest.get_visible_trees();

    println!("{visible_trees}");
}

fn get_height_matrix(input: String) -> Matrix<u32> {
    let mut height_matrix = Vec::new();

    // Get input lines
    let lines = input.lines().into_iter();

    for tree_line in lines {
        let mut height_row: Vec<u32> = Vec::new();

        for height in tree_line.chars() {
            height_row.push(height.to_digit(10).expect("NaN"));
        }

        height_matrix.push(height_row);
    }

    return height_matrix;
}
