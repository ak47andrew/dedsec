mod constants;
mod utils;

use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use colored::Colorize;
use rand::{rng, Rng};
use crate::constants::{BANNER, ERROR_LINES, INIT_LINES, WARNING_LINES, WELCOME_LINE};
use crate::utils::echo::EchoUtil;
use crate::utils::utilabc::UtilAbc;

fn main() {
    let mut random = rng();

    // Banner
    println!("{}", BANNER.bright_green().bold());

    // Starting lines
    let mut init_lines = INIT_LINES.to_vec();
    while init_lines.len() > 0 {
        let ind = random.random_range(0..init_lines.len());
        println!("{} {}", "[+]".cyan().bold(), init_lines[ind].bright_green().italic());
        sleep(Duration::from_millis(random.random_range(50..150)));
        if random.random::<f32>() < 0.05 {
            println!("{} {}", "[*]".yellow().bold(), WARNING_LINES[random.random_range(0..WARNING_LINES.len())].bright_green().italic());
            sleep(Duration::from_millis(random.random_range(150..350)));
        }
        if random.random::<f32>() < 0.05 {
            println!("{} {}", "[!]".red().bold(), ERROR_LINES[random.random_range(0..ERROR_LINES.len())].bright_green().italic());
            sleep(Duration::from_millis(random.random_range(250..500)));
        } else {
            init_lines.remove(ind);
        }
    }
    println!("{}", WELCOME_LINE.cyan());

    let utils: [Box<dyn UtilAbc>; 1] = [
        Box::new(EchoUtil {})
    ];

    // Main loop
    loop {
        for (ind, util) in utils.iter().enumerate() {
            println!("{}) {}", ind + 1, util.get_name())
        }
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let index: usize;
        match input.trim().parse::<usize>() {
            Err(_) => {
                println!("{}", "Unknown command".red());
                continue;
            },
            Ok(i) => index = i,
        }

        if index == 0 || index - 1 >= utils.len() {
            println!("{}", "Unknown command".red());
            continue;
        }

        utils[index - 1].run();
    }
}
