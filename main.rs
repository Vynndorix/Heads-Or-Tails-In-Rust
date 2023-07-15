use rand::Rng;
use std::io;

fn main(){

    loop{
        println!("Heads or Tails? ");

        let tt = rand::thread_rng().gen_range(0..=1);

        let mut usr = String::new();
        
        io::stdin().read_line(&mut usr).expect("error");

        let usr_choice = match usr.trim().to_lowercase().as_str() {
            "heads" => 0,
            "tails" => 1,
            _ => {
                println!("Error");
                return;
            }
        };

        let result = if tt == usr_choice {
            "You Win!"
        } else {
            "You Lose!"
        };

        println!("The coin landed on {}.", if tt == 0 { "heads" } else { "tails" });
        println!("{}", result);
    }

}
