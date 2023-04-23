use std::vec;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const CELL_SIZE: f32 = 50.;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Board {
    SnakeHead(Entity),
    SnakeBody(Entity),
    Empty
}
#[derive(Resource)]
pub struct SnakeBoard {
    board: Vec<Vec<Board>>,
} 
impl SnakeBoard {

    pub fn new(width: usize, height: usize) -> Self {
        let mut new_board = Self {
            board: Vec::new()
        };

        for x in 0..width/CELL_SIZE as usize {
            new_board.board.push(Vec::new());
            for _ in 0..height/CELL_SIZE as usize {
                new_board.board[x].push(Board::Empty);
            }
        }

        let middle = new_board.board.len()/2;
        let other_middle = new_board.board[middle].len()/2;
        for i in 0..4 {
            new_board.board[middle][other_middle - i] = 
                if i == 0 { Board::SnakeHead(Entity::PLACEHOLDER) }
                else { Board::SnakeBody(Entity::PLACEHOLDER) };
        }

        new_board
    }

    pub fn start(&mut self) -> Vec<(usize, usize)> {
        let middle: usize = self.board.len()/2;
        let other_middle: usize = self.board[middle].len()/2;
        let mut references: Vec<(usize, usize)> = Vec::new();
        for i in 0..4 {
            references.push(((other_middle - i) as usize, middle));
        }
        references
    }
}


pub fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_board: ResMut<SnakeBoard>,
) {
    cmd.spawn(Camera2dBundle::default());
    
    // Build Board
    let mut color = Color::DARK_GRAY;

    for x in ((-WINDOW_WIDTH/2.) as isize..(WINDOW_WIDTH/2.) as isize)
        .map(|i| i as f32).filter(|f| f%CELL_SIZE == 0.) {

        color = if color == Color::DARK_GRAY { Color::GRAY } else { Color::DARK_GRAY };

        for y in ((-WINDOW_HEIGHT/2.) as isize..(WINDOW_HEIGHT/2.) as isize)
            .map(|i| i as f32).filter(|f| f%CELL_SIZE == 0.) {

            color = if color == Color::DARK_GRAY { Color::GRAY } else { Color::DARK_GRAY };

            cmd.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(CELL_SIZE, CELL_SIZE)).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_xyz(x+CELL_SIZE/2., y+CELL_SIZE/2., 0.),
                ..Default::default()
            });
        }
    }
    
    let middle: usize = snake_board.board.len()/2;
    for (x, y) in snake_board.start() {
        let entity = cmd.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(CELL_SIZE, CELL_SIZE)).into()).into(),
            material: materials.add(ColorMaterial::from(
                if x == middle { Color::GREEN } else { Color::DARK_GREEN }
            )),
            transform: Transform::from_xyz(
                -WINDOW_WIDTH/2. + (x as f32) * CELL_SIZE - CELL_SIZE/2.,
                -WINDOW_HEIGHT/2. + (middle as f32) * CELL_SIZE - CELL_SIZE/2.,
                1.
                ),
            ..Default::default()
        }).id();
        
        if snake_board.board[x][y] == Board::SnakeHead(Entity::PLACEHOLDER) {
            snake_board.board[x][y] = Board::SnakeHead(entity);
        } else {
            snake_board.board[x][y] = Board::SnakeBody(entity);
        }
    }
    
}
