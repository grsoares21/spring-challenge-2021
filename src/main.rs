//mod ai_adhoc;
mod ai_greedy_with_heuristic;
mod easing_functions;
mod game_logic;
mod input_parsing;

use game_logic::*;
use input_parsing::*;

fn main() {
    let initial_input = parse_initial_input();

    // game loop
    loop {
        let turn_input = parse_turn_input(&initial_input.cells);
        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>

        eprintln!("DAY: {}", turn_input.day);

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

        //let chosen_action = ai_adhoc::get_next_action(current_state, possible_actions);
        let chosen_action =
            ai_greedy_with_heuristic::get_next_action(current_state, possible_actions);

        println!("{}", action_to_order(chosen_action));
    }
}
