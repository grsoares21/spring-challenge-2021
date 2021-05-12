use crate::game_logic::{Action, GameState, Tree};

pub fn get_next_action(game_state: GameState, possible_actions: Vec<Action>) -> Action {
  let grown_trees: Vec<&Tree> = game_state
    .my_trees
    .iter()
    .filter(|tree| tree.size == 3)
    .collect();

  let harvestable_number_of_trees = game_state.sunpoints / 4;
  let needed_days_for_harvesting = match harvestable_number_of_trees {
    0 => i32::MAX,
    _ => (grown_trees.len() as i32 / harvestable_number_of_trees) + 1,
  };

  if grown_trees.len() > 0
    && (game_state.sunpoints > 15
      || (game_state.day >= 24 - needed_days_for_harvesting && harvestable_number_of_trees > 0))
  {
    return Action::Complete(grown_trees.first().unwrap().cell_index);
  } else if possible_actions.iter().any(|action| match action {
    Action::Seed(_, _) => true,
    _ => false,
  }) && game_state.my_trees.len() < 8
    && !game_state.my_trees.iter().any(|tree| tree.size == 0)
  {
    let mut seed_actions: Vec<&Action> = possible_actions
      .iter()
      .filter(|action| match action {
        Action::Seed(_, _) => true,
        _ => false,
      })
      .collect();

    seed_actions.sort_by(|action_a, action_b| match (action_a, action_b) {
      (Action::Seed(_, dest_a), Action::Seed(_, dest_b)) => game_state.cells[*dest_a as usize]
        .richness
        .cmp(&game_state.cells[*dest_b as usize].richness),
      _ => panic!("Invalid seed action"),
    });

    return *seed_actions[0];
  } else if possible_actions.iter().any(|action| match action {
    Action::Grow(_) => true,
    _ => false,
  }) {
    let mut grow_actions: Vec<&Action> = possible_actions
      .iter()
      .filter(|action| match action {
        Action::Grow(_) => true,
        _ => false,
      })
      .collect();

    grow_actions.sort_by(|action_a, action_b| match (action_a, action_b) {
      (Action::Grow(dest_a), Action::Grow(dest_b)) => game_state.cells[*dest_a as usize]
        .richness
        .cmp(&game_state.cells[*dest_b as usize].richness),
      _ => panic!("Invalid grow action"),
    });

    return *grow_actions[0];
  } else {
    return Action::Wait;
  }
}
