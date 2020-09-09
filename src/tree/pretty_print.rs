use crate::tree::*;

use std::fmt::{self, Display, Write};

fn build_space(x: u32) -> String {
    let mut res = String::new();
    for _ in 0..x {
        write!(&mut res, "  ").unwrap();
    }
    res
}

fn build_indent(level: u32, _height: u32) -> String {
    let indent = 2u32.pow(level) - 1;
    build_space(indent)
}

fn build_spacer(level: u32, _height: u32) -> String {
    let spacer = 2u32.pow(level) - 1;
    build_space(spacer)
}


impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.get_height() + 1;
        write!(f, "Tree Height: {}\n", height)?;
        write!(f, "Root: {}\n", self.get_root())?;
        write!(f, "\n")?;
        for i in (0..height).rev() {
            let level = self.get_level(i);
            for node in level.iter() {
                write!(
                    f,
                    "{}{}{}",
                    build_indent(i + 1, height),
                    node,
                    build_spacer(i + 1, height)
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04?}", self.id)
    }
}
