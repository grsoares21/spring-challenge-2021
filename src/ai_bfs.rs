//use crate::easing_functions::*;
use crate::game_logic::*;
use std::collections::HashSet;
use std::time::SystemTime;

pub fn get_estimated_sunpoint_rate(game_state: &GameState) -> f32 {
  let mut total_gathered_sun = 0.0;

  for i in 1..=6 {
    let shadows = get_maxed_out_shadows_in_field(
      game_state
        .my_trees
        .iter()
        .chain(game_state.opponent_trees.iter()),
      game_state.day + i,
      &game_state.cells,
    );

    let total_gathered = game_state
      .my_trees
      .iter()
      .map(|tree| {
        return (if tree.size > shadows[tree.cell_index as usize] {
          (tree.size + 2) as f32
        } else {
          2.0
        }) / i as f32;
      }) // the + 2 here is only to incentivze seeds
      .fold(0.0, |a, b| a + b);

    //eprintln!("Total gathered on day {}: {}", game_state.day + i, total_gathered);

    total_gathered_sun += total_gathered;
  }

  // eprintln!("Total gathered: {}", total_gathered_sun);

  return total_gathered_sun / 6.0;
}

pub fn get_estimated_sunpoint_rate_for_enemy(game_state: &GameState) -> f32 {
  let mut total_gathered_sun = 0.0;

  for i in 1..=6 {
    let shadows = get_shadows_in_field(
      game_state
        .my_trees
        .iter()
        .chain(game_state.opponent_trees.iter()),
      game_state.day + 1,
      &game_state.cells,
    );

    let total_gathered = game_state
      .opponent_trees
      .iter()
      .map(|tree| {
        return (if tree.size > shadows[tree.cell_index as usize] {
          (tree.size) as f32
        } else {
          0.0
        }) / i as f32;
      }) // the + 2 here is only to incentivze seeds
      .fold(0.0, |a, b| a + b);

    //eprintln!("Total gathered on day {}: {}", game_state.day + i, total_gathered);

    total_gathered_sun += total_gathered;
  }

  // eprintln!("Total gathered: {}", total_gathered_sun);

  return total_gathered_sun / 6.0;
}

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
        return tree.size;
      } else {
        return 0;
      }
    }) // the + 2 here is only to incentivze seeds
    .fold(0, |a, b| a + b);
}

pub fn get_richness_score(game_state: &GameState) -> i32 {
  return game_state
    .my_trees
    .iter()
    .map(|tree| (tree.cell.richness + 3))
    .fold(0, |a, b| a + b);
}

pub fn evaluate_state(game_state: &GameState, print_calculations: bool) -> f32 {
  let sunpoint_rate = get_estimated_sunpoint_rate(game_state);
  let enemy_sunpoint_rate = get_estimated_sunpoint_rate_for_enemy(game_state);

  let game_completion_factor = (game_state.day as f32 / 23.0).powf(3.0);
  let score_valuation = (1.0 + game_state.score as f32).powf(game_completion_factor);
  //let enemy_score_valuation = (1.0 + game_state.opponent_score as f32).powf(game_completion_factor);
  let sunrate_valuation = (1.0 + sunpoint_rate).powf((1 as f32) - game_completion_factor);
  let enemy_sunrate_valuation =
    (1.0 + enemy_sunpoint_rate).powf((1 as f32) - game_completion_factor);
  let richness_score = get_richness_score(game_state) as f32;

  let scp = score_valuation;
  let srp = sunrate_valuation - (enemy_sunrate_valuation * (1.0 - game_completion_factor));

  let state_value = scp * srp + (richness_score / 10000.0) + (game_state.sunpoints as f32 / 1000.0);

  if print_calculations {
    eprintln!(
      "sr: {}, esr: {}, sc: {}, esc: {}, srv: {}, esrv: {}, sp: {}, rs:{}, scp: {}, srp:{}, t:{}",
      sunpoint_rate,
      enemy_sunpoint_rate,
      game_state.score,
      game_state.opponent_score,
      sunrate_valuation,
      enemy_sunrate_valuation,
      game_state.sunpoints,
      richness_score,
      scp,
      srp,
      state_value
    );
  }

  return state_value;
}

pub fn simulate_action(game_state: &GameState, action: Action) -> GameState {
  return match action {
    Action::Wait => {
      let mut new_game_state = game_state.clone();

      // todo same for enemy
      for mut tree in new_game_state.my_trees.iter_mut() {
        tree.is_dormant = false;
      }

      new_game_state.sunpoints += get_sunpoint_rate(&new_game_state);
      if new_game_state.day < 23 {
        new_game_state.day += 1;
      }
      // increase day nu
      new_game_state
    }
    Action::Grow(target) => {
      let mut new_game_state = game_state.clone();
      let mut tree_to_grow = new_game_state
        .my_trees
        .iter_mut()
        .find(|tree| tree.cell_index == target)
        .unwrap();

      tree_to_grow.size = tree_to_grow.size + 1;
      tree_to_grow.is_dormant = true;
      new_game_state.sunpoints -= get_sun_cost_to_grow(tree_to_grow.size, &game_state.my_trees);

      return new_game_state;
    }
    Action::Seed(source_tree, target_cell) => {
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

      let mut tree_to_launch_seed = new_game_state
        .my_trees
        .iter_mut()
        .find(|tree| tree.cell_index == source_tree)
        .unwrap();

      tree_to_launch_seed.is_dormant = true;

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
  let mut chosen_plan = &ListOfActionsForDay {
    game_state: game_state.clone(),
    actions: vec![Action::Wait],
    length: 1,
  };
  let mut current_score = f32::MIN;
  //eprintln!("Evaluated score for action WAIT: {}", current_score);

  let possible_list_of_actions = get_possible_actions_for_day(&game_state, seedable_cells);

  eprintln!(
    "Number of possible actions for day {}: {}",
    game_state.day,
    possible_list_of_actions.len()
  );

  for (i, list_of_actions) in possible_list_of_actions.iter().enumerate() {
    let should_print = i < 1
      || match list_of_actions.actions.first().unwrap() {
        Action::Wait => true,
        _ => false,
      };
    if should_print {
      let action_orders: Vec<String> = list_of_actions
        .actions
        .iter()
        .map(|action| action_to_order(*action))
        .collect();
      eprintln!("{}", action_orders.join(" "));
    }
    let new_state_score = evaluate_state(&list_of_actions.game_state, should_print);

    if new_state_score >= current_score {
      chosen_plan = list_of_actions;
      current_score = new_state_score;
    }
  }

  evaluate_state(&chosen_plan.game_state, true);

  *chosen_plan.actions.first().unwrap()
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
  if game_state.day == 0 {
    return possible_actions;
  }

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
      && !(tree.size == 0 && number_of_trees_of_size[(tree.size + 1) as usize] >= 2)
      && !(tree.size == 1 && number_of_trees_of_size[(tree.size + 1) as usize] >= 5)
    {
      possible_actions.push(Action::Grow(tree.cell_index));
    }
  }
  if game_state.day == 1 {
    return possible_actions;
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
          if tree.cell_index == 33 {
            // eprintln!("Seedable cell for 33: {}", cell);
          }
          if !unusable_cells.contains(&cell) {
            //eprintln!("Adding action SEED 33 {}", cell);
            possible_actions.push(Action::Seed(tree.cell_index, *cell));
          }
        }
      }
    }
  }

  return possible_actions;
}

pub struct ListOfActionsForDay {
  game_state: GameState,
  actions: Vec<Action>,
  length: i32,
}

pub fn get_possible_actions_for_day(
  game_state: &GameState,
  seedable_cells: &Vec<Vec<Vec<i32>>>,
) -> Vec<ListOfActionsForDay> {
  let mut possible_actions_until_wait: Vec<ListOfActionsForDay> = Vec::new();

  let mut possible_actions_until_wait_queue: Vec<ListOfActionsForDay> = Vec::new();

  let initial_possible_actions = get_possible_actions(game_state, seedable_cells);

  for action in initial_possible_actions {
    possible_actions_until_wait_queue.push(ListOfActionsForDay {
      game_state: simulate_action(game_state, action),
      actions: vec![action],
      length: 0,
    })
  }

  while possible_actions_until_wait_queue.len() > 0 {
    let visiting_list = possible_actions_until_wait_queue.pop().unwrap();
    let last_action = visiting_list.actions.last().unwrap();

    match last_action {
      Action::Wait => {
        possible_actions_until_wait.push(visiting_list);
      }
      _ => {
        if visiting_list.length > 1 {
          let new_game_state = simulate_action(&visiting_list.game_state, Action::Wait);
          let mut new_actions: Vec<Action> = visiting_list.actions.iter().copied().collect();

          new_actions.push(Action::Wait);

          let updated_visiting_list = ListOfActionsForDay {
            game_state: new_game_state,
            actions: new_actions,
            length: visiting_list.length + 1,
          };

          possible_actions_until_wait.push(updated_visiting_list);
        } else {
          let possible_actions = get_possible_actions(&visiting_list.game_state, seedable_cells);

          for action in possible_actions {
            let new_game_state = simulate_action(&visiting_list.game_state, action);
            let mut new_actions: Vec<Action> = visiting_list.actions.iter().copied().collect();

            new_actions.push(action);

            let updated_visiting_list = ListOfActionsForDay {
              game_state: new_game_state,
              actions: new_actions,
              length: visiting_list.length + 1,
            };
            possible_actions_until_wait_queue.push(updated_visiting_list);
          }
        }
      }
    }
  }

  return possible_actions_until_wait;
}
