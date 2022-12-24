use crate::Mainstruct;
pub fn pushfn(selfx: &mut Mainstruct, letter: String) {
    selfx.letters.push(shiftfn(selfx.shift, letter.to_string()));
    //println!("ADDED {} TO {}", letter,LETTERS.lock_mut().unwrap().concat());
    
    selfx.colour = 0;
}

pub fn shiftfn(shift: bool, letter: String) -> String {
    if shift == true {
        return letter.to_uppercase();
    } else {
        return letter.to_lowercase();
    }
}

pub fn shiftvaluefn(selfx: &mut Mainstruct) {
    if selfx.shift == true {
        selfx.shift = false ;
    } else {
        selfx.shift = true;
    }
}
pub fn popfn(selfx: &mut Mainstruct) {
    if selfx.letters.len() != 0 {
        selfx.letters.pop();
        //println!("{}",LETTERS.lock_mut().unwrap().concat());
        selfx.colour = 0;
    } 
    
}