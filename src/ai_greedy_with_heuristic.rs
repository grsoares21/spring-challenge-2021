//use crate::easing_functions::*;
use crate::game_logic::*;

pub fn get_sun_cost_to_score_ratio(target_tree: &Tree, game_state: &GameState) -> f32 {
  return get_score_for_cell(game_state.nutrients, &target_tree.cell) as f32
    / get_sun_cost_to_completion(target_tree, &game_state.my_trees) as f32;
}

pub fn get_sunpoint_rate(my_trees: &Vec<Tree>) -> i32 {
  return my_trees
    .iter()
    .map(|tree| tree.size + 2) // the + 2 here s only to incentivze seeds
    .fold(0, |a, b| a + b);
}

pub fn evaluate_state(game_state: &GameState) -> f32 {
  let sun_cost_to_score_ratio = if game_state.my_trees.len() > 0 {
    (game_state
      .my_trees
      .iter()
      .map(|tree| get_sun_cost_to_score_ratio(tree, game_state))
      .fold(1 as f32, |a, b| a + b.powf(3.0))
      / game_state.my_trees.len() as f32)
      .powf(1.0 / 3.0)
  } else {
    0.0
  };

  let normalized_sunpoint_rate =
    get_sunpoint_rate(&game_state.my_trees) as f32 * sun_cost_to_score_ratio;

  let game_completion_factor = (game_state.day as f32 / 23.0).powf(3.0);

  eprintln!(
    "Game completion percentage: {}, game completion factor: {}",
    game_state.day as f32 / 23.0,
    game_completion_factor
  );
  /*eprintln!(
      "Sunpoint rate: {}, sun to score ratio: {}, score: {}, game completion %: {}, points for score: {}, points for sunrate: {}",
      normalized_sunpoint_rate,
      sun_cost_to_score_ratio,
      game_state.score,
      game_completion_factor,
      game_state.score as f32 * game_completion_factor,
      (normalized_sunpoint_rate as f32) * ((1 as f32) - game_completion_factor)
  );*/

  let score_valuation = (1.0 + game_state.score as f32).powf(game_completion_factor);
  let sunrate_valuation =
    (1.0 + normalized_sunpoint_rate as f32).powf((1 as f32) - game_completion_factor);

  eprintln!(
    "Score valuation: {} - Sun rate valuation: {}",
    score_valuation, sunrate_valuation
  );

  return score_valuation * sunrate_valuation;
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

pub fn get_next_action(game_state: GameState, possible_actions: Vec<Action>) -> Action {
  let shadows = get_shadows_in_field(
    game_state
      .my_trees
      .iter()
      .chain(game_state.opponent_trees.iter()),
    game_state.day,
    &game_state.cells,
  );
  let mut chosen_action = Action::Wait;
  let mut current_score = evaluate_state(&game_state);
  let number_of_seeds = game_state
    .my_trees
    .iter()
    .filter(|tree| tree.size == 0)
    .count();
  eprintln!("Evaluated score for action WAIT: {}", current_score);

  for possible_action in possible_actions {
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
