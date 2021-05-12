use crate::types::{Cell, Tree};
use std::io;

pub struct InitialInput {
  pub number_of_cells: i32,
  pub cells: Vec<Cell>,
}

pub struct TurnInput {
  pub day: i32,
  pub nutrients: i32,
  pub sunpoints: i32,
  pub score: i32,
  pub opponent_sunpoints: i32,
  pub opponent_score: i32,
  pub opponent_is_waiting: bool,
  pub my_trees: Vec<Tree>,
  pub opponent_trees: Vec<Tree>,
  pub possible_actions: Vec<String>,
}

#[macro_export]
macro_rules! parse_input {
  ($x:expr, $t:ident) => {
    $x.trim().parse::<$t>().unwrap()
  };
}

pub fn parse_initial_input() -> InitialInput {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();

  let number_of_cells = parse_input!(input_line, i32);

  let mut cells: Vec<Cell> = Vec::with_capacity(37);

  for _ in 0..number_of_cells as usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let index = parse_input!(inputs[0], i32); // 0 is the center cell, the next cells spiral outwards
    let richness = parse_input!(inputs[1], i32); // 0 if the cell is unusable, 1-3 for usable cells
    let _neigh_0 = parse_input!(inputs[2], i32); // the index of the neighbouring cell for each direction
    let _neigh_1 = parse_input!(inputs[3], i32);
    let _neigh_2 = parse_input!(inputs[4], i32);
    let _neigh_3 = parse_input!(inputs[5], i32);
    let _neigh_4 = parse_input!(inputs[6], i32);
    let _neigh_5 = parse_input!(inputs[7], i32);

    cells.push(Cell {
      index: index,
      richness: richness,
      neighbours: [_neigh_0, _neigh_1, _neigh_2, _neigh_3, _neigh_4, _neigh_5],
    })
  }

  return InitialInput {
    number_of_cells: number_of_cells,
    cells: cells,
  };
}

pub fn parse_turn_input(cells: &Vec<Cell>) -> TurnInput {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let inputs = input_line.split(" ").collect::<Vec<_>>();
  let sun = parse_input!(inputs[0], i32); // your sun points
  let score = parse_input!(inputs[1], i32); // your current score
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let inputs = input_line.split(" ").collect::<Vec<_>>();
  let opp_sun = parse_input!(inputs[0], i32); // opponent's sun points
  let opp_score = parse_input!(inputs[1], i32); // opponent's score
  let opp_is_waiting = parse_input!(inputs[2], i32); // whether your opponent is asleep until the next day
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let number_of_trees = parse_input!(input_line, i32); // the current amount of trees

  let mut my_trees: Vec<Tree> = Vec::new();
  let mut opponent_trees: Vec<Tree> = Vec::new();

  for _ in 0..number_of_trees as usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let cell_index = parse_input!(inputs[0], i32); // location of this tree
    let size = parse_input!(inputs[1], i32); // size of this tree: 0-3
    let is_mine = parse_input!(inputs[2], i32); // 1 if this is your tree
    let is_dormant = parse_input!(inputs[3], i32); // 1 if this tree is dormant

    let tree = Tree {
      cell_index: cell_index,
      is_mine: match is_mine {
        1 => true,
        _ => false,
      },
      is_dormant: match is_dormant {
        1 => true,
        _ => false,
      },
      size: size,
      cell: cells[cell_index as usize],
    };

    if tree.is_mine {
      my_trees.push(tree);
    } else {
      opponent_trees.push(tree);
    }
  }

  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let number_of_possible_actions = parse_input!(input_line, i32); // all legal actions

  let mut possible_actions: Vec<String> = Vec::with_capacity(number_of_possible_actions as usize);

  for _ in 0..number_of_possible_actions as usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let possible_action = input_line.trim_matches('\n').to_string(); // try printing something from here to start with
    eprintln!("possible action: {}", possible_action);

    possible_actions.push(possible_action);
  }

  TurnInput {
    day: day,
    nutrients: nutrients,
    sunpoints: sun,
    score: score,
    opponent_sunpoints: opp_sun,
    opponent_score: opp_score,
    opponent_is_waiting: match opp_is_waiting {
      1 => true,
      _ => false,
    },
    my_trees: my_trees,
    opponent_trees: opponent_trees,
    possible_actions: possible_actions,
  }
}

/*
USED only for ad-hoc algorithm
pub fn parse_seed_actions(seed_actions: Vec<&String>) -> Vec<SeedAction> {
  return seed_actions
    .iter()
    .map(|action| {
      let action_parts: Vec<&str> = action.split(" ").collect();
      return SeedAction {
        tree_index: parse_input!(action_parts[1], i32),
        cell_index: parse_input!(action_parts[2], i32),
      };
    })
    .collect();
}

pub fn parse_grow_actions(grow_actions: Vec<&String>, turn_input: &TurnInput) -> Vec<GrowAction> {
  return grow_actions
    .iter()
    .map(|action| {
      let action_parts: Vec<&str> = action.split(" ").collect();
      let tree_cell_index = parse_input!(action_parts[1], i32);
      let tree = turn_input
        .my_trees
        .iter()
        .find(|tree| tree.cell_index == tree_cell_index)
        .unwrap();
      return GrowAction { tree: *tree };
    })
    .collect();
}
*/
