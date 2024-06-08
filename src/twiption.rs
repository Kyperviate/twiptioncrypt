mod constants;

use constants::GREEK_ABECEDARIES;
use constants::SQRT_DIGITS;

pub fn encrypt(pt: &str, key: &str) -> String {
    let mut ret = String::new();
    let key_v: Vec<char> = key.chars().collect();
    let sd = SQRT_DIGITS[pt.len()];
    for (i, c) in pt.chars().enumerate() {
	ret.push(
	    encrypt_letter(c, key_v[i % key.len()], sd[i])
	);
    }
    
    return ret;
}

#[allow(dead_code)]
pub fn decrypt(ct: &str, key: &str) -> String {
    let mut ret = String::new();
    let key_v: Vec<char> = key.chars().collect();
    let sd = SQRT_DIGITS[ct.len()];
    for (i, c) in ct.chars().enumerate() {
	ret.push(
	    decrypt_letter(c, key_v[i % key.len()] as char, sd[i])
	);
    }
    return ret;
}

fn encrypt_letter(pt: char, key: char, abecedary_key: usize) -> char {
    let a = GREEK_ABECEDARIES[abecedary_key];
    let l_pos = a.find(pt).unwrap();
    let k_pos = a.find(key).unwrap();
    let ret = a.as_bytes()[(l_pos + k_pos) % 26] as char;
    return ret;
}

#[allow(dead_code)]
fn decrypt_letter(ct: char, key: char, abecedary_key: usize) -> char {
    let a = GREEK_ABECEDARIES[abecedary_key];
    let mut l_pos = a.find(ct).unwrap();
    let k_pos = a.find(key).unwrap();
    l_pos += 26;
    let ret = a.as_bytes()[(l_pos - k_pos) % 26] as char;
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encrypts() {
	let ptext = "HOWNOWBROWNCOWDOESTHESUNSETATNOOB";
	let key = "AXE";
	let expected_ctext = "KLWHSWCJOTKKBSPBXUIBTSLOWXTDPOBFI";
	assert_eq!(encrypt(&ptext, &key), expected_ctext);
    }

    #[test]
    fn it_decrypts() {
	let ptext = "HOWNOWBROWNCOWDOESTHESUNSETATNOOB";
	let key = "AXE";
	let ctext = encrypt(&ptext, &key);
	assert_eq!(ptext, decrypt(&ctext, &key));
    }
}
