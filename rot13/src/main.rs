use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
// impl<R: Read> Read for RotDecoder<R> {
//     fn read(&mut self, buf: &mut String) -> io::Result<usize> {
//         self.rot.read_to_string(buf)
//     }
// }

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.input.read(buf)?;
        for b in &mut buf[..size] {
            if b.is_ascii_alphabetic() {
                let base = if b.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                *b = (*b - base + self.rot) % 26 + base;
            }
        }
        Ok(size)
    }
}


// Implement a random seed that implement  RngCore https://rust-random.github.io/rand/rand_core/trait.RngCore.html
impl<R: Read> RngCore for RotDecoder<R> {
    fn next_u32(&mut self) -> u32{
        13
    }

    fn next_u64(&mut self) -> u64{
        13
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]);
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error>;
}

fn main() {
    let mut rot = RotDecoder {
        input: "Gb trg gb gur bgure fvqr!".as_bytes(),
        rot: 13,
    };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder {
            input: "Gb trg gb gur bgure fvqr!".as_bytes(),
            rot: 13,
        };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> {
            input: input.as_ref(),
            rot: 13,
        };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
