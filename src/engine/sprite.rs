use std::fs::read_to_string;
use std::path::Path;
use anyhow::{anyhow, Error};
use vector2d::Vector2D;
use crate::engine::drawable::Drawable;
use crate::engine::rendering::frame::Frame;

pub struct Sprite {
    pub label: String,
    data: Vec<Vec<char>>,
    pub visible: bool,
    pub translation: Vector2D<usize>,
    pub layer: i32,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl Sprite {

    pub fn from_string(label: String, string: String) -> Self {
        let mut data = vec![];
        let row = Sprite::string_to_utf8(string.as_str());
        data.push(row);

        Self {
            label,
            visible: true,
            data: transpose(data),
            translation: Vector2D::new(0, 0),
            layer: 0,
        }
    }

    pub fn from_file<P: AsRef<Path>>(label: String, path: P) -> Result<Sprite, Error> {
        if let Ok(content) = read_to_string(path) {
            let lines: Vec<&str> = content.lines().collect();
            let height = lines.len();
            let width = lines.iter()
                .map(|line| line.len())
                .max()
                .unwrap();

            let mut data = Vec::with_capacity(height);
            for line in lines.iter() {
                let mut row = vec![' '; width];
                for y in 0..width {
                    if let Some(char) = line.chars().nth(y) {
                        row[y] = char;
                    }
                }
                data.push(row);
            }
            println!("{}", data.is_empty());

            Ok(Self {
                label,
                visible: true,
                data: transpose(data),
                translation: Vector2D::new(0, 0),
                layer: 0,
            })

        } else {
            Err(anyhow!("Could not load sprite from file."))
        }

    }

    fn string_to_utf8(str: &str) -> Vec<char> {
        let mut row = vec![];
        for c in str.chars() {
            row.push(c);
        }
        row
    }
}

impl Drawable for Sprite {
    fn draw(&self, frame: &mut Frame) {
        if self.visible {
            for (x, col) in self.data.iter().enumerate() {
                for (y, char) in col.iter().enumerate() {
                    let x_pos = self.translation.x + x;
                    let y_post = self.translation.y + y;
                    frame[x_pos][y_post] = *char as u32
                }
            }
        }
    }
}