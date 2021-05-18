use std::collections::HashMap;

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
  pub opponent_score: i32,
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
      opponent_score: self.opponent_score,
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

pub fn get_sun_cost_to_completion_from_size(from_size: i32, my_trees: &Vec<Tree>) -> i32 {
  let mut current_size = from_size;
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
    for cell in get_tree_shadow_cells(current_tree, cells, shadow_direction) {
      if shadows[cell as usize] < current_tree.size {
        shadows[cell as usize] = current_tree.size
      }
    }
  }

  return shadows;
}

pub fn get_maxed_out_shadows_in_field<'a>(
  trees: impl Iterator<Item = &'a Tree>,
  day: i32,
  cells: &Vec<Cell>,
) -> [i32; 37] {
  let mut shadows = [0; 37];
  let shadow_direction = (day % 6) as usize;

  for current_tree in trees {
    for cell in get_maxed_out_tree_shadow_cells(current_tree.cell_index, cells, shadow_direction) {
      shadows[cell as usize] = 3
    }
  }

  return shadows;
}

pub fn get_maxed_out_shadows_in_field_with_simulated_trees<'a>(
  trees: impl Iterator<Item = &'a Tree>,
  day: i32,
  cells: &Vec<Cell>,
) -> [i32; 37] {
  let mut shadows = [0; 37];
  let shadow_direction = (day % 6) as usize;

  for current_tree in trees {
    for cell in get_maxed_out_tree_shadow_cells(current_tree.cell_index, cells, shadow_direction) {
      shadows[cell as usize] = 3
    }
  }

  for i in 0..37 {
    for cell in get_maxed_out_tree_shadow_cells(i, cells, shadow_direction) {
      if shadows[cell as usize] < 1 {
        shadows[cell as usize] = 1
      }
    }
  }

  return shadows;
}

pub fn get_maxed_out_shadows_in_field_with_real_size<'a>(
  trees: impl Iterator<Item = &'a Tree>,
  day: i32,
  cells: &Vec<Cell>,
) -> [i32; 37] {
  let mut shadows = [0; 37];
  let shadow_direction = (day % 6) as usize;

  for current_tree in trees {
    for cell in get_maxed_out_tree_shadow_cells(current_tree.cell_index, cells, shadow_direction) {
      if shadows[cell as usize] < current_tree.size {
        shadows[cell as usize] = current_tree.size
      }
    }
  }

  return shadows;
}

pub fn get_maxed_out_tree_shadow_cells(
  cell_index: i32,
  cells: &Vec<Cell>,
  direction: usize,
) -> Vec<i32> {
  let mut cells_within_reach: Vec<i32> = Vec::new();

  let mut current_neighbour = cells[cell_index as usize].neighbours[direction];
  for _ in 0..3 as usize {
    if current_neighbour == -1 {
      break;
    }

    cells_within_reach.push(current_neighbour);
    current_neighbour = cells[current_neighbour as usize].neighbours[direction];
  }

  return cells_within_reach;
}

pub fn get_tree_shadow_cells(tree: &Tree, cells: &Vec<Cell>, direction: usize) -> Vec<i32> {
  let mut cells_within_reach: Vec<i32> = Vec::new();

  let mut current_neighbour = tree.cell.neighbours[direction];
  for _ in 0..tree.size as usize {
    if current_neighbour == -1 {
      break;
    }

    cells_within_reach.push(current_neighbour);
    current_neighbour = cells[current_neighbour as usize].neighbours[direction];
  }

  return cells_within_reach;
}

// too path-finding with max distance equal to the trees size
pub fn get_tree_seedable_cells(tree: &Tree, cells: &Vec<Cell>) -> Vec<i32> {
  struct CellToVisit {
    cell_index: i32,
    distance: i32,
  }

  let mut cells_to_visit: Vec<CellToVisit> = Vec::new();
  let mut lowest_distance_to_cells: HashMap<i32, i32> = HashMap::new();

  for neighbour in &tree.cell.neighbours {
    if *neighbour != -1 {
      cells_to_visit.push(CellToVisit {
        distance: 1,
        cell_index: *neighbour,
      });
      lowest_distance_to_cells.insert(*neighbour, 1);
    }
  }

  while cells_to_visit.len() > 0 {
    let cell_to_visit = cells_to_visit.pop().unwrap();

    if cell_to_visit.distance + 1 <= tree.size {
      for neighbour in &cells[cell_to_visit.cell_index as usize].neighbours {
        let lowset_distance_to_cell = lowest_distance_to_cells.get(neighbour);
        let is_shortest_path = match lowset_distance_to_cell {
          Some(distance) => *distance > cell_to_visit.distance + 1,
          None => true,
        };

        if *neighbour != -1 && *neighbour != tree.cell_index && is_shortest_path {
          cells_to_visit.push(CellToVisit {
            distance: cell_to_visit.distance + 1,
            cell_index: *neighbour,
          });
          lowest_distance_to_cells.insert(*neighbour, cell_to_visit.distance + 1);
        }
      }
    }
  }

  return lowest_distance_to_cells.iter().map(|(k, _)| *k).collect();
}

pub fn get_seedable_cells_for_cell_and_size(
  tree_cell: usize,
  tree_size: i32,
  cells: &Vec<Cell>,
) -> Vec<i32> {
  struct CellToVisit {
    cell_index: i32,
    distance: i32,
  }

  let mut cells_to_visit: Vec<CellToVisit> = Vec::new();
  let mut lowest_distance_to_cells: HashMap<i32, i32> = HashMap::new();

  for neighbour in cells[tree_cell].neighbours.iter() {
    if *neighbour != -1 {
      cells_to_visit.push(CellToVisit {
        distance: 1,
        cell_index: *neighbour,
      });
      lowest_distance_to_cells.insert(*neighbour, 1);
    }
  }

  while cells_to_visit.len() > 0 {
    let cell_to_visit = cells_to_visit.pop().unwrap();

    if cell_to_visit.distance + 1 <= tree_size {
      for neighbour in &cells[cell_to_visit.cell_index as usize].neighbours {
        let lowset_distance_to_cell = lowest_distance_to_cells.get(neighbour);
        let is_shortest_path = match lowset_distance_to_cell {
          Some(distance) => *distance > cell_to_visit.distance + 1,
          None => true,
        };

        if *neighbour != -1 && *neighbour != tree_cell as i32 && is_shortest_path {
          cells_to_visit.push(CellToVisit {
            distance: cell_to_visit.distance + 1,
            cell_index: *neighbour,
          });
          lowest_distance_to_cells.insert(*neighbour, cell_to_visit.distance + 1);
        }
      }
    }
  }

  return lowest_distance_to_cells.iter().map(|(k, _)| *k).collect();
}
