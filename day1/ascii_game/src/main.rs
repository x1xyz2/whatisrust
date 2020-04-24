extern crate rand;
use rand::Rng;
use ncurses::*;
fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(0, 128);
    let secret_char: char = secret_number as char;

    initscr();
    refresh();

    println!("secret '{}', ascii-code {}\r\n", secret_char, secret_number);
    loop {
        addstr("Let's try : ");
        let choice_num = getch() as u8;
   
        if secret_number > choice_num {
            addstr("\nLow!!\n");
        }
        else if secret_number < choice_num {
            addstr("\nHigh!!\n");
        }
        else {
            addstr("\nGood Job!!!, Press anykey to Exit");
            getch();
            break;
        }
    }
    endwin();
}

