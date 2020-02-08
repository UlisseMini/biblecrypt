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

fn decrypt() -> std::io::Result<()> {
    for chunk in std::io::stdin()
            .lock()
            .bytes()
            .map(|x| x.unwrap())
            .collect::<Vec<u8>>()
            .chunks(8) {


        // let chunk = [chunk, &[0,0,0,0]].concat();
        let mut array = [0; 8];
        array.copy_from_slice(&chunk);

        let idx: usize = usize::from_le_bytes(array);
        std::io::stdout().write(&[BIBLE.as_bytes()[idx]]).unwrap();
    }
    Ok(())
}

fn encrypt() -> std::io::Result<()> {
    let map = biblemap_new();

    let mut rng = rand::thread_rng();

    for b in std::io::stdin().lock().bytes() {
        let b = b?;
        match map.get(&b) {
            Some(v) => {
                let idx = v.choose(&mut rng).unwrap();
                let bytes = &idx.to_le_bytes();
                assert_eq!(BIBLE.as_bytes()[*idx], b);
                std::io::stdout().write(bytes).unwrap();
            },
            None => panic!(format!("cannot find char {} in the king james bible", b)),
        }
    }
    Ok(())
}

fn help() {
    println!("Usage: biblecrypt <encrypt/decrypt>")
}

fn main() {
    let cmd = std::env::args().skip(1).next();
    match cmd {
        Some(s) => {
            // TODO: figure out why matching on strings did not work
            if s == "encrypt" { encrypt().unwrap() }
            else { decrypt().unwrap() }
        },
        None => help(),
    };
}
