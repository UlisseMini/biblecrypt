/* Bible encryption steps
 * Step 1. Build data structure of every character in the bible and its locations
 * Step 2. For every plaintext char find the index of that character in the bible and store it it
 */


/* Bible decryption steps
 * For every index in the input lookup that index in the bible
 */

use std::collections::HashMap;
use rand::seq::SliceRandom;
use std::io::{Read, Write};
use rand::rngs::ThreadRng;
use std::io;

static BIBLE: &'static str = include_str!("the-king-james-bible.txt");


type BibleMap = HashMap<u8, Vec<usize>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biblemap() {
        let map = biblemap_new();

        // TODO: Refactor to use bytes for O(1) indexing
        for (key, vals) in map.iter() {
            for val in vals {
                assert_eq!(key, &BIBLE.as_bytes()[*val]);
            }
        }
    }

    #[test]
    fn test_encrypt_decrypt() {
        // encrypt
        let mut bc = BibleCipher::new();
        let encrypted = bc.encrypt('h' as u8).unwrap();

        // decrypt
        let mut array = [0; 8];
        array.copy_from_slice(&encrypted);
        let decrypted = bc.decrypt(array);

        // assert
        assert_eq!(decrypted, 'h' as u8);
    }
}

fn biblemap_new() -> BibleMap {
    let mut map: BibleMap = HashMap::new();
    for (i, c) in BIBLE.bytes().enumerate() {
        match map.get_mut(&c) {
            Some(v) => v.push(i),
            None => {map.insert(c, vec![i]);},
        };
    }
    map
}

struct BibleCipher {
    map: BibleMap,
    rng: ThreadRng,
}

impl BibleCipher {
    fn new() -> BibleCipher {
        BibleCipher {
            map: biblemap_new(),
            rng: rand::thread_rng(),
        }
    }

    /// Encrypt b using the bible, if b is not in the bible None will be returned.
    fn encrypt(&mut self, b: u8) -> Option<[u8; 8]> {
        match self.map.get(&b) {
            Some(v) => {
                let idx = v.choose(&mut self.rng).unwrap();
                Some(idx.to_le_bytes())
            },
            None => None,
        }
    }

    fn decrypt(&self, chunk: [u8; 8]) -> u8 {
        // let chunk = [chunk, &[0,0,0,0]].concat();

        let idx: usize = usize::from_le_bytes(chunk);
        BIBLE.as_bytes()[idx]
    }
}

fn help() {
    println!("Usage: biblecrypt <encrypt/decrypt>")
}

fn main() -> io::Result<()> {
    let cmd = std::env::args().skip(1).next();
    match cmd {
        Some(s) => {
            let mut bc = BibleCipher::new();

            if s == "encrypt" || s == "e" || s == "enc" {
                for b in io::stdin().lock().bytes() {
                    let b = b?;
                    match bc.encrypt(b) {
                        Some(bytes) => {
                            io::stdout().write(&bytes)?;
                        },
                        None => {
                            // TODO: Move into run() function and return an error here
                            eprintln!("Could not find {} in the holy bible", b);
                            return Ok(());
                        }
                    }
                }
            } else {
                for chunk in io::stdin()
                    .lock()
                    .bytes()
                    .map(|x| x.unwrap())
                    .collect::<Vec<u8>>()
                    .chunks(8) {

                    let mut array = [0; 8];
                    array.copy_from_slice(&chunk);
                    bc.decrypt(array);
               }
            }
        },
        None => help(),
    };
    Ok(())
}
