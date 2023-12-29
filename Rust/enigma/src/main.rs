use std::{io, str::FromStr};

macro_rules! read_line_as {
    ($t:ty) => {{
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line.trim().parse::<$t>().unwrap()
    }};
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Encode,
    Decode,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ENCODE" => Ok(Action::Encode),
            "DECODE" => Ok(Action::Decode),
            _ => Err(()),
        }
    }
}

const ALPHABET_LEN: usize = 26;
const FIRST_LETTER_IN_ALPHABET: u8 = b'A';

#[derive(Debug, Default, Clone)]
struct Rotor {
    alphabet_index_at: [u8; ALPHABET_LEN],
    position_of: [u8; ALPHABET_LEN],
}

impl FromStr for Rotor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != ALPHABET_LEN {
            return Err(());
        }
        let mut alphabet_index_at = [0; ALPHABET_LEN];
        let mut position_of = [0; ALPHABET_LEN];
        for (i, byte) in s.as_bytes().into_iter().enumerate() {
            let alphabet_index = byte.checked_sub(FIRST_LETTER_IN_ALPHABET);
            match alphabet_index {
                Some(alphabet_index) if alphabet_index < ALPHABET_LEN as u8 => {
                    alphabet_index_at[i] = alphabet_index;
                    position_of[alphabet_index as usize] = i as u8;
                }
                _ => return Err(()),
            }
        }

        Ok(Rotor {
            alphabet_index_at,
            position_of,
        })
    }
}

impl Rotor {
    fn alphabet_index_at(&self, position: u8) -> u8 {
        self.alphabet_index_at[position as usize]
    }

    fn position_of(&self, x: u8) -> u8 {
        self.position_of[x as usize]
    }
}

fn ceasar_shift(x: u8, shift: u8, i: usize, action: Action) -> u8 {
    let shift = ((shift as usize + i) % ALPHABET_LEN) as u8;
    let n = ALPHABET_LEN as u8;
    match action {
        Action::Encode => (x + shift) % n,
        Action::Decode => (n + x - shift) % n,
    }
}

fn apply_rotor(x: u8, rotor: &Rotor, action: Action) -> u8 {
    match action {
        Action::Encode => rotor.alphabet_index_at(x),
        Action::Decode => rotor.position_of(x),
    }
}

fn apply_rotors<'a>(x: u8, rotors: impl Iterator<Item = &'a Rotor>, action: Action) -> u8 {
    let mut result = x;
    for rotor in rotors {
        result = apply_rotor(result, rotor, action);
    }
    result
}

fn to_string(iter: impl Iterator<Item = u8>) -> String {
    iter.map(|x| (x + FIRST_LETTER_IN_ALPHABET) as char)
        .collect::<String>()
}

fn enigma_cipher(message: &[u8], shift: u8, rotors: &[Rotor], action: Action) -> String {
    let alphabet_index_iter = message.iter().map(|x| x - FIRST_LETTER_IN_ALPHABET);
    match action {
        Action::Encode => to_string(
            alphabet_index_iter
                .enumerate()
                .map(|(i, x)| ceasar_shift(x, shift, i, action))
                .map(|x| apply_rotors(x, rotors.iter(), action)),
        ),
        Action::Decode => to_string(
            alphabet_index_iter
                .map(|x| apply_rotors(x, rotors.iter().rev(), action))
                .enumerate()
                .map(|(i, x)| ceasar_shift(x, shift, i, action)),
        ),
    }
}

fn main() {
    let action = read_line_as!(Action);
    let shift = read_line_as!(u8);
    let mut rotors: [Rotor; 3] = Default::default();
    for i in 0..3 {
        rotors[i] = read_line_as!(Rotor);
    }
    let message = read_line_as!(String);

    println!(
        "{}",
        enigma_cipher(message.as_bytes(), shift, &rotors, action)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    const PLAIN_TEXT: &[u8; 3] = b"AAA";
    const CIPHER_TEXT: &[u8; 3] = b"KQF";
    const SHIFT: u8 = 4;
    const ROTOR_1: &str = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
    const ROTOR_2: &str = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
    const ROTOR_3: &str = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";

    #[test]
    fn test_encode() {
        let rotors = [
            ROTOR_1.parse::<Rotor>().unwrap(),
            ROTOR_2.parse::<Rotor>().unwrap(),
            ROTOR_3.parse::<Rotor>().unwrap(),
        ];
        assert_eq!(
            enigma_cipher(PLAIN_TEXT, SHIFT, &rotors, Action::Encode),
            str::from_utf8(CIPHER_TEXT).unwrap()
        );
    }

    #[test]
    fn test_decode() {
        let rotors = [
            ROTOR_1.parse::<Rotor>().unwrap(),
            ROTOR_2.parse::<Rotor>().unwrap(),
            ROTOR_3.parse::<Rotor>().unwrap(),
        ];
        assert_eq!(
            enigma_cipher(CIPHER_TEXT, SHIFT, &rotors, Action::Decode),
            str::from_utf8(PLAIN_TEXT).unwrap()
        );
    }
}
