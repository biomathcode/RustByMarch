pub struct RandGen {
    curr: usize,
    mul:usize, 
    inc: usize, 
    modulo: usize, 
}

impl RandGen {
    pub fn new(curr:usize) -> Self{
        RandGen{
            curr, 
            mul: 4823432, 
            
        }
    }
}