mod input_parsing;
mod types;

use input_parsing::*;
use types::*;

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

pub fn get_sun_cost_to_score_ratio(target_tree: &Tree, game_state: &GameState) -> f32 {
    return get_score_for_cell(game_state.nutrients, &target_tree.cell) as f32
        / get_sun_cost_to_completion(target_tree, &game_state.my_trees) as f32;
}

pub fn get_sunpoint_rate(my_trees: &Vec<Tree>) -> i32 {
    return my_trees
        .iter()
        .map(|tree| tree.size + 1)
        .fold(0, |a, b| a + b);
}

/**
*
* Sun points rate tem + importância, quanto mais o day esta proximo de 0

* Plantar arvores é melhor quando temos poucas arvores e pior quando temos muitas
* Crescer arvores é melhor quando temos muitas arvores e pior quando temos poucas
*
* Desempate:
*
* Desempate:
* Plantar em solo rico é melhor do que plantar em solo pobre
* Crescer arvore em solo rico é melhor do que crescer em solo pobre

* Score tem + importância, quanto mais o day esta proximo de 24
*/

/* custo teorico para score */

pub fn evaluate_state(game_state: &GameState) -> f32 {
    let sun_cost_to_score_ratio = (game_state
        .my_trees
        .iter()
        .map(|tree| get_sun_cost_to_score_ratio(tree, game_state))
        .fold(1 as f32, |a, b| a + b.powf(3.0))
        / game_state.my_trees.len() as f32)
        .powf(1.0 / 3.0);

    let normalized_sunpoint_rate =
        get_sunpoint_rate(&game_state.my_trees) as f32 * sun_cost_to_score_ratio;

    let game_completion_percentage = game_state.day as f32 / 23 as f32;

    eprintln!(
        "Sunpoint rate: {}, sun to score ratio: {}, score: {}, game completion %: {}, points for score: {}, points for sunrate: {}",
        normalized_sunpoint_rate,
        sun_cost_to_score_ratio,
        game_state.score,
        game_completion_percentage,
        game_state.score as f32 * game_completion_percentage.powf(2.0),
        (normalized_sunpoint_rate as f32) * ((1 as f32) - game_completion_percentage.powf(2.0))
    );

    return game_state.score as f32 * game_completion_percentage.powf(2.0)
        + (normalized_sunpoint_rate as f32) * ((1 as f32) - game_completion_percentage.powf(2.0));
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
            new_game_state.sunpoints -=
                get_sun_cost_to_grow(tree_to_grow.size, &game_state.my_trees);

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

pub struct GameState {
    pub cells: Vec<Cell>,
    pub day: i32,
    pub score: i32,
    pub nutrients: i32,
    pub sunpoints: i32,
    pub my_trees: Vec<Tree>,
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
        }
    }
}

/** ONLY USED FOR AD-HOC algorithm
 *  fn grow_tree_in_richest_soil(mut possible_grow_actions: Vec<GrowAction>) {
    possible_grow_actions.sort_by(|action_a: &GrowAction, action_b: &GrowAction| {
        return (action_b.tree.cell.richness * 1000 + action_b.tree.size)
            .cmp(&(action_a.tree.cell.richness * 1000 + action_a.tree.size));
    });

    for action in &possible_grow_actions {
        eprintln!(
            "Grow index: {}, Richness: {}, Size: {}",
            action.tree.cell_index, action.tree.cell.richness, action.tree.size
        );
    }

    println!("GROW {}", possible_grow_actions[0].tree.cell_index);
}
*/

fn main() {
    // 37
    let initial_input = parse_initial_input();

    // game loop
    loop {
        let turn_input = parse_turn_input(&initial_input.cells);
        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>

        eprintln!("DAY: {}", turn_input.day);

        /*
        ONLY used in ad-hoc algorithm
        let grown_trees: Vec<&Tree> = turn_input
            .my_trees
            .iter()
            .filter(|tree| tree.size == 3)
            .collect();

        let possible_seed_actions: Vec<&String> = turn_input
            .possible_actions
            .iter()
            .filter(|action| action.contains("SEED"))
            .collect();

        let possible_grow_actions: Vec<&String> = turn_input
            .possible_actions
            .iter()
            .filter(|action| action.contains("GROW"))
            .collect();

            */

        let possible_actions: Vec<Action> = turn_input
            .possible_actions
            .iter()
            .map(|action_string| {
                let action_parts: Vec<&str> = action_string.split(" ").collect();
                return match action_parts[0] {
                    "COMPLETE" => {
                        let target = parse_input!(action_parts[1], i32);
                        return Action::Complete(target);
                    }
                    "SEED" => {
                        let source = parse_input!(action_parts[1], i32);
                        let target = parse_input!(action_parts[2], i32);
                        return Action::Seed(source, target);
                    }
                    "WAIT" => Action::Wait,
                    "GROW" => {
                        let target = parse_input!(action_parts[1], i32);
                        return Action::Grow(target);
                    }
                    _ => panic!("Parseou tudo errado"),
                };
            })
            .collect();

        let current_state = GameState {
            cells: initial_input.cells.iter().copied().collect(),
            day: turn_input.day,
            score: turn_input.score,
            nutrients: turn_input.nutrients,
            sunpoints: turn_input.sunpoints,
            my_trees: turn_input.my_trees,
        };

        let mut chosen_action = Action::Wait;
        let mut current_score = evaluate_state(&current_state);
        eprintln!("Evaluated score for action WAIT: {}", current_score);

        for (i, possible_action) in possible_actions.iter().enumerate() {
            match *possible_action {
                Action::Wait => {
                    continue;
                }
                _ => {
                    eprintln!(
                        "Evaluating score for action {}....",
                        turn_input.possible_actions[i]
                    );
                    let new_state_with_action = simulate_action(&current_state, *possible_action);
                    let new_state_score = evaluate_state(&new_state_with_action);

                    eprintln!(
                        "Evaluated score for action {}: {}",
                        turn_input.possible_actions[i], new_state_score
                    );

                    if new_state_score > current_score {
                        chosen_action = *possible_action;
                        current_score = new_state_score;
                    }
                }
            }
        }

        match chosen_action {
            Action::Wait => println!("WAIT"),
            Action::Grow(target) => println!("GROW {}", target),
            Action::Seed(source, target) => println!("SEED {} {}", source, target),
            Action::Complete(target) => println!("COMPLETE {}", target),
        }
        /*
        AD-HOC algorithm
        let harvestable_number_of_trees = turn_input.sunpoints / 4;
        let needed_days_for_harvesting = match harvestable_number_of_trees {
            0 => i32::MAX,
            _ => (grown_trees.len() as i32 / harvestable_number_of_trees) + 1,
        };

        if grown_trees.len() > 0
            && (turn_input.sunpoints > 15
                || (turn_input.day >= 24 - needed_days_for_harvesting
                    && harvestable_number_of_trees > 0))
        {
            println!("COMPLETE {}", grown_trees.first().unwrap().cell_index);
        } else if possible_seed_actions.len() > 0
            && turn_input.my_trees.len() < 8
            && !turn_input.my_trees.iter().any(|tree| tree.size == 0)
        {
            let mut seed_actions = parse_seed_actions(possible_seed_actions);
            seed_actions.sort_by(|action_a: &SeedAction, action_b: &SeedAction| {
                return initial_input.cells[action_b.cell_index as usize]
                    .richness
                    .cmp(&initial_input.cells[action_a.cell_index as usize].richness);
            });

            let chosen_seed_action = &seed_actions[0];

            println!(
                "SEED {} {}",
                chosen_seed_action.tree_index, chosen_seed_action.cell_index
            )
        } else if possible_grow_actions.len() > 0 {
            let mut grow_actions = parse_grow_actions(possible_grow_actions, &turn_input);

            grow_tree_in_richest_soil(grow_actions);
        } else {
            println!("WAIT");
        }*/
    }
}
