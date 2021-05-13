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

pub struct GameState {
  pub cells: Vec<Cell>,
  pub day: i32,
  pub score: i32,
  pub nutrients: i32,
  pub sunpoints: i32,
  pub my_trees: Vec<Tree>,
  pub opponent_trees: Vec<Tree>,
}

impl Clone for GameState {
  fn clone(&self) -> GameState {
    GameState {
      day: self.day,
      score: self.score,
      nutrients: self.nutrients,
      sunpoints: self.sunpoints,
      cells: self.cells.iter().copied().collect(),
      my_trees: self.my_trees.iter().copied().collect(),
      opponent_trees: self.opponent_trees.iter().copied().collect(),
    }
  }
}

pub fn get_sun_cost_to_grow(new_size: i32, my_trees: &Vec<Tree>) -> i32 {
  let number_of_target_size_trees = my_trees
    .iter()
    .map(|tree| if tree.size == new_size { 1 } else { 0 })
    .reduce(|a, b| a + b)
    .unwrap();

  return number_of_target_size_trees
    + match new_size {
      0 => 0,
      1 => 1,
      2 => 3,
      3 => 7,
      _ => panic!("Invalid tree size: {}", new_size),
    };
}

pub fn get_sun_cost_to_completion(target_tree: &Tree, my_trees: &Vec<Tree>) -> i32 {
  let mut current_size = target_tree.size;
  let mut current_cost = 0;
  while current_size < 3 {
    current_size += 1;

    current_cost += get_sun_cost_to_grow(current_size, my_trees)
  }

  return current_cost + 4;
}

pub fn get_score_for_cell(nutrients: i32, cell: &Cell) -> i32 {
  return nutrients + (2 * (cell.richness - 1));
}

pub fn action_to_order(action: Action) -> String {
  match action {
    Action::Wait => format!("WAIT"),
    Action::Grow(target) => format!("GROW {}", target),
    Action::Seed(source, target) => format!("SEED {} {}", source, target),
    Action::Complete(target) => format!("COMPLETE {}", target),
  }
}

pub fn get_shadows_in_field<'a>(
  trees: impl Iterator<Item = &'a Tree>,
  day: i32,
  cells: &Vec<Cell>,
) -> [i32; 37] {
  let mut shadows = [0; 37];
  let shadow_direction = (day % 6) as usize;

  for current_tree in trees {
    let mut current_neighbour = current_tree.cell.neighbours[shadow_direction];
    for _ in 0..current_tree.size as usize {
      if current_neighbour == -1 {
        break;
      }

      if shadows[current_neighbour as usize] < current_tree.size {
        shadows[current_neighbour as usize] = current_tree.size
      }

      current_neighbour = cells[current_neighbour as usize].neighbours[shadow_direction];
    }
  }

  for i in 0..37 as usize {
    eprintln!("Shadow size in cell {}: {}", i, shadows[i]);
  }

  return shadows;
}
