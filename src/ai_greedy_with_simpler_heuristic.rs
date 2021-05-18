//use crate::easing_functions::*;
use crate::game_logic::*;
use std::collections::HashSet;

pub fn get_sunpoint_rate(game_state: &GameState) -> i32 {
  let shadows = get_shadows_in_field(
    game_state
      .my_trees
      .iter()
      .chain(game_state.opponent_trees.iter()),
    game_state.day + 1,
    &game_state.cells,
  );

  return game_state
    .my_trees
    .iter()
    .map(|tree| {
      if tree.size > shadows[tree.cell_index as usize] {
        return tree.size + 2;
      } else {
        return 2;
      }
    }) // the + 2 here is only to incentivze seeds
    .fold(0, |a, b| a + b);
}

pub fn get_richness_score(game_state: &GameState) -> i32 {
  return game_state
    .my_trees
    .iter()
    .map(|tree| (tree.size + 1) * (tree.cell.richness + 3))
    .fold(0, |a, b| a + b);
}

pub fn evaluate_state(game_state: &GameState) -> f32 {
  let sunpoint_rate = get_sunpoint_rate(game_state) as f32;

  let game_completion_factor = (game_state.day as f32 / 23.0).powf(5.0);
  let score_valuation = (1.0 + game_state.score as f32).powf(game_completion_factor);
  let sunrate_valuation = (1.0 + sunpoint_rate).powf((1 as f32) - game_completion_factor);
  let richness_score = get_richness_score(game_state);

  let state_value = score_valuation * sunrate_valuation + richness_score as f32 / 10000.0;
  return state_value;
}

pub fn simulate_action(game_state: &GameState, action: Action) -> GameState {
  return match action {
    Action::Wait => game_state.clone(),
    Action::Grow(target) => {
      let mut new_game_state = game_state.clone();
      let mut tree_to_grow = new_game_state
        .my_trees
        .iter_mut()
        .find(|tree| tree.cell_index == target)
        .unwrap();

      tree_to_grow.size = tree_to_grow.size + 1;
      new_game_state.sunpoints -= get_sun_cost_to_grow(tree_to_grow.size, &game_state.my_trees);

      return new_game_state;
    }
    Action::Seed(_, target_cell) => {
      // first param is source_tree
      let mut new_game_state = game_state.clone();
      new_game_state.my_trees.push(Tree {
        cell_index: target_cell,
        size: 0,
        is_mine: true,
        is_dormant: true,
        cell: *new_game_state
          .cells
          .iter()
          .find(|cell| cell.index == target_cell)
          .unwrap(),
      });

      // TODO: source_tree becomes dorment
      get_sun_cost_to_grow(0, &game_state.my_trees);

      return new_game_state;
    }
    Action::Complete(target) => {
      let mut new_game_state = game_state.clone();
      let tree_index = new_game_state
        .my_trees
        .iter()
        .position(|tree| tree.cell_index == target)
        .unwrap();

      new_game_state.score += get_score_for_cell(
        game_state.nutrients,
        &new_game_state.my_trees[tree_index].cell,
      );
      new_game_state.my_trees.remove(tree_index);
      new_game_state.sunpoints -= 4;

      return new_game_state;
    }
  };
}

pub fn get_next_action(game_state: GameState, seedable_cells: &Vec<Vec<Vec<i32>>>) -> Action {
  let mut chosen_action = Action::Wait;
  let mut current_score = evaluate_state(&game_state);
  let number_of_seeds = game_state
    .my_trees
    .iter()
    .filter(|tree| tree.size == 0)
    .count();
  eprintln!("Evaluated score for action WAIT: {}", current_score);

  for possible_action in get_possible_actions(&game_state, seedable_cells) {
    match possible_action {
      Action::Wait => {
        continue;
      }
      _ => {
        if match possible_action {
          Action::Seed(_, _) => true,
          _ => false,
        } && number_of_seeds > 0
        {
          continue;
        }
        eprintln!(
          "Evaluating score for action {}....",
          action_to_order(possible_action)
        );
        let new_state_with_action = simulate_action(&game_state, possible_action);
        let new_state_score = evaluate_state(&new_state_with_action);

        eprintln!(
          "Evaluated score for action {}: {}",
          action_to_order(possible_action),
          new_state_score
        );

        if new_state_score > current_score {
          chosen_action = possible_action;
          current_score = new_state_score;
        }
      }
    }
  }

  chosen_action
}

pub fn get_possible_actions(
  game_state: &GameState,
  seedable_cells: &Vec<Vec<Vec<i32>>>,
) -> Vec<Action> {
  let mut possible_actions = Vec::new();

  let mut number_of_trees_of_size = [0; 4];
  for tree in &game_state.my_trees {
    number_of_trees_of_size[tree.size as usize] += 1;
  }

  possible_actions.push(Action::Wait);

  // GROW & COMPLETE
  for tree in &game_state.my_trees {
    if tree.is_dormant {
      continue;
    }

    if tree.size == 3 && game_state.sunpoints >= 4 {
      possible_actions.push(Action::Complete(tree.cell_index));
    }

    if tree.size < 3
      && get_sun_cost_to_grow(tree.size + 1, &game_state.my_trees) <= game_state.sunpoints
      && !(tree.size == 0 && number_of_trees_of_size[(tree.size + 1) as usize] >= 1)
      && !(tree.size == 1 && number_of_trees_of_size[(tree.size + 1) as usize] >= 2)
    {
      possible_actions.push(Action::Grow(tree.cell_index));
    }
  }

  // SEEDS
  // VALUES used

  if number_of_trees_of_size[0] == 0 {
    // only consider seeding if there's no seed on field
    let mut unusable_cells: HashSet<i32> = HashSet::new();
    for cell in &game_state.cells {
      if cell.richness == 0 {
        unusable_cells.insert(cell.index);
      }
    }
    for tree in game_state
      .my_trees
      .iter()
      .chain(game_state.opponent_trees.iter())
    {
      unusable_cells.insert(tree.cell_index);
    }

    for tree in &game_state.my_trees {
      //eprintln!("Checking seeds for tree on: {}", tree.cell_index);
      if tree.size > 0 && !tree.is_dormant {
        for cell in &seedable_cells[tree.cell_index as usize][(tree.size - 1) as usize] {
          if !unusable_cells.contains(&cell) {
            possible_actions.push(Action::Seed(tree.cell_index, *cell));
          }
        }
      }
    }
  }

  return possible_actions;
}
