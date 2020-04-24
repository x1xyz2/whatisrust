extern crate rand;
use rand::Rng;
use ncurses::*;
fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(0, 128);
    let secret_char: char = secret_number as char;
    println!("secret_number {}, secret_char '{}'", secret_number, secret_char);

    let mut try_count = 1;
    initscr();
    refresh();

    println!("secret_number {}, secret_char '{}'", secret_number, secret_char);
    loop {
        addstr("Let's try : ");
        let choice_num = getch() as u8;
        let choice_char = choice_num as char;
        println!("\n[#{}] your choice is '{}', ascii-code {}", try_count, choice_char, choice_num);
        try_count = try_count + 1;
   
        if secret_number > choice_num {
            addstr("Up!!");
        }
        else if secret_number < choice_num {
            addstr("Down!!");
        }
        else {
            addstr("You're Right!!!, Press anykey to Exit");
            getch();
            break;
        }
    }
    endwin();
}

