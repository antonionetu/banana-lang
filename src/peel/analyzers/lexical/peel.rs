use crate::lib;

pub fn throw(peel: String) {
    let mut can = String::new();
    let mut trash = String::new();
    let mut throwing = false;

    for p in peel.chars() {
        if p == '(' {
            if throwing {
                // recursive
                // throw();
            }

            throwing = true;
            continue;
        } else if p == ')' {
            throwing = false;
            continue;
        }

        if throwing {
            trash.push(p);
        } else {
            can.push(p);
        }
    }

    organic_waste_bin(can, trash);
}

fn organic_waste_bin(can: String, trash: String){
    if can == "say"{
        lib::functions::console::say(trash);
    } // elif ...
}
