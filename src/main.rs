mod ai_greedy_with_simpler_heuristic;
mod game_logic;
mod input_parsing;

use game_logic::*;
use input_parsing::*;
use std::time::SystemTime;

fn main() {
    let initial_input = parse_initial_input();

    let mut seedable_cells: Vec<Vec<Vec<i32>>> = Vec::with_capacity(37);

    for cell in 0..37 {
        seedable_cells.push(Vec::with_capacity(3));

        for size in 0..3 {
            seedable_cells[cell].push(get_seedable_cells_for_cell_and_size(
                cell,
                size as i32 + 1,
                &initial_input.cells,
            ));
        }
    }

    // game loop
    loop {
        let turn_input = parse_turn_input(&initial_input.cells);

        eprintln!("DAY: {}", turn_input.day);

        let current_state = GameState {
            cells: initial_input.cells.iter().copied().collect(),
            day: turn_input.day,
            score: turn_input.score,
            nutrients: turn_input.nutrients,
            sunpoints: turn_input.sunpoints,
            my_trees: turn_input.my_trees,
            opponent_trees: turn_input.opponent_trees,
            opponent_score: turn_input.opponent_score,
        };

        //let chosen_action = ai_adhoc::get_next_action(current_state, possible_actions);
        let now = SystemTime::now();
        let chosen_action =
            ai_greedy_with_simpler_heuristic::get_next_action(current_state, &seedable_cells);
        match now.elapsed() {
            Ok(elapsed) => {
                // it prints '2'
                eprintln!("elapsed {}", elapsed.as_millis());
            }
            Err(e) => {
                // an error occurred!
                eprintln!("Error: {:?}", e);
            }
        }

        println!("{}", action_to_order(chosen_action));
    }
}
