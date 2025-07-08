mod utils;


fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        utils::prompt::display_prompt();
        
        input = utils::input::get_input();
        println!("You typed: {}", input.trim());
    }

}
