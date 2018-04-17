extern crate difference;

use self::difference::*;


pub fn get_diff(current: String, new: String) -> String{
    let changeset = Changeset::new(&current, &new, " ");

    let mut new_menu = String::new();
    for change in changeset.diffs{
        match change{
            Difference::Same(word) => {
                new_menu.push_str(&format!(" {}", &word.trim_left()));
            },
            Difference::Add(word) => {

                new_menu.push_str(&format!(" *{}*", &word.trim_left()));
            },
            Difference::Rem(_) => {

            }
        }
    }
    new_menu
}

