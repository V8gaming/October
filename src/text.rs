use crate::Mainstruct;
pub fn pushfn(selfx: &mut Mainstruct, letter: String) {
    selfx.letters.push(shiftfn(selfx.shift, letter));
    //println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());

    selfx.colour = 0;
}

pub fn shiftfn(shift: bool, letter: String) -> String {
    if shift {
        letter.to_uppercase()
    } else {
        letter.to_lowercase()
    }
}

pub fn shiftvaluefn(selfx: &mut Mainstruct) {
    if selfx.shift {
        selfx.shift = false;
    } else {
        selfx.shift = true;
    }
}
pub fn popfn(selfx: &mut Mainstruct) {
    if !selfx.letters.is_empty() {
        selfx.letters.pop();
        //println!("{}",LETTERS.lock_mut().unwrap().concat());
        selfx.colour = 0;
    }
}
