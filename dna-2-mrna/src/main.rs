use std::io;
#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (index, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'T' | 'G' => continue,
                _ => return Err(index),
            }
        }

        Ok(Dna {
            dna: String::from(dna),
        })
    }
    pub fn into_rna(self) -> Rna {
        let mut rna = String::new();
        for c in self.dna.chars() {
            match c {
                'A' => rna.push('U'),
                'C' => rna.push('G'),
                'G' => rna.push('C'),
                'T' => rna.push('A'),
                _ => continue,
            }
        }
        Rna { rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (index, c) in rna.chars().enumerate() {
            match c {
                'A' | 'C' | 'T' | 'G' => continue,
                _ => return Err(index),
            }
        }

        Ok(Rna {
            rna: String::from(rna),
        })
    }
}

fn main() {
    // let mut input = String::new();

    // io::stdin().read_line(&mut input).unwrap();
    // let char_vec: Vec<char> = input.chars().collect();

    // let mut new_char = String::new();

    // for c in char_vec {
    //     match c {
    //         'a' => new_char += "u",
    //         't' => new_char += "a",
    //         'g' => new_char += "c",
    //         'c' => new_char += "g",
    //         _ => new_char += &c.to_string(),
    //     }
    // }
    // println!("{}", new_char);
}
