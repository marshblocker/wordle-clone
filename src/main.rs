// Wordle made in Rust. (wow very original!)

extern crate colored;

use std::process;

use wordle_clone::wordbank::{WordBank};
use wordle_clone::user_guess;
use wordle_clone::guess_processor::{process_guess, IfWinner};
use wordle_clone::display::{self, Display, GuessColorMapping};
use wordle_clone::constants::MAX_GUESSES;

fn main() {
    display::clear_screen();

    let mut display: Display = Display::init();
    let wordbank = WordBank::init();
                           
    let mut winner: IfWinner = false;

    let mut guesses_left = MAX_GUESSES;
    let unknown_word: String = wordbank.get_random_word_in_unknown_words();

    let cmd: char = display::display_start_screen();
    if cmd == 'H' { display::display_help(); }

    display::clear_screen();
    display.print_allowed_letters();
    display.print_all_guesses();
    display::display_guesses_left(guesses_left as u8);

    while guesses_left > 0 {
        let guess: String = user_guess::get_user_guess(&wordbank)
                                                    .unwrap_or_else(|err| {
                                                        eprintln!("{}", err);
                                                        process::exit(1);
                                                    });
        let (gcm, winner_temp): (GuessColorMapping, IfWinner) = process_guess(
            &guess, &unknown_word, &mut display);

        winner = winner_temp;

        display::clear_screen();
        
        display.update_allowed_letters();
        display.get_user_guess(gcm);
        display.update_user_guess_arr(MAX_GUESSES - guesses_left);
        guesses_left -= 1;
        display.print_allowed_letters();
        display.print_all_guesses();

        if winner { display::display_end_screen(winner, &unknown_word); } 
        
        display::display_guesses_left(guesses_left as u8);

    }
    
    display::display_end_screen(winner, &unknown_word);
}
