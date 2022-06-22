
pub struct Mutator {

}

impl Mutator {
    pub fn mutate(input: Vec<u8>){
        let &mut ran = rand::thread_rng();
        let b:u8 = rand::thread_rng().gen();
        std::mem::replace(&mut input[ran], b);
    }

}