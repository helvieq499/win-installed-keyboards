use win_installed_keyboards::LanguageID;

fn main() {
    for (index, keyboard) in win_installed_keyboards::list_keyboards()
        .unwrap()
        .iter()
        .enumerate()
    {
        match keyboard.get_language_id() {
            // ID was recognised and not a missing LCID
            Some(LanguageID::Tagged(x)) => println!("Keyboard {index} is {:?}", x),

            // ID was recognised as a missing LCID
            Some(_) => println!("Keyboard {index} has a temporary LCID"),

            // ID was not recognised at all
            None => println!("Keyboard {index} has an unknown LCID"),
        }
    }
}
