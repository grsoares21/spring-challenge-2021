#[derive(Clone)]
pub struct Cell {
  pub index: i32,
  pub richness: i32,
  pub neighbours: [i32; 6],
}

impl Copy for Cell {}

#[derive(Clone)]
pub struct Tree {
  pub cell_index: i32,
  pub size: i32,
  pub is_mine: bool,
  pub is_dormant: bool,
  pub cell: Cell,
}

impl Copy for Tree {}

#[derive(Clone, Copy)]
pub enum Action {
  Wait,
  Grow(i32),
  Complete(i32),
  Seed(i32, i32),
}

/*
Only used in ad hoc algorithm

pub struct SeedAction {
  pub cell_index: i32,
  pub tree_index: i32,
}

pub struct GrowAction {
  pub tree: Tree,
}
*/
