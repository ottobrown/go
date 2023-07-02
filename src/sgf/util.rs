#![allow(unused)]

use super::SgfError;
use super::SgfResult;

pub fn to_sgf_coord(x: usize) -> SgfResult<char> {
    if x <= 25 {
        return Ok((x as u8 + 97) as char);
    } else if x <= 51 {
        return Ok((x as u8 + 39) as char);
    }

    Err(SgfError::CoordTooBig)
}

pub fn from_sgf_coord(c: char) -> SgfResult<usize> {
    let i = c as usize;

    if i < 65 {
        return Err(SgfError::InvalidCoordChar);
    } else if i <= 90 {
        return Ok(i - 39);
    } else if i <= 122 {
        return Ok(i - 97);
    }

    Err(SgfError::InvalidCoordChar)
}

/// Converts the first two characters of the string into coordinates
pub fn string_coords(s: &str) -> SgfResult<(usize, usize)> {
    let mut ch = s.chars();

    Ok((
        from_sgf_coord(ch.next().unwrap())?,
        from_sgf_coord(ch.next().unwrap())?,
    ))
}

#[test]
fn to_coord_test() {
    assert_eq!(to_sgf_coord(0).unwrap(), 'a');
    assert_eq!(to_sgf_coord(25).unwrap(), 'z');
    assert_eq!(to_sgf_coord(26).unwrap(), 'A');
    assert_eq!(to_sgf_coord(51).unwrap(), 'Z');
    assert!(to_sgf_coord(52).is_err());
}

#[test]
fn from_coord_test() {
    assert_eq!(from_sgf_coord('a').unwrap(), 0);
    assert_eq!(from_sgf_coord('z').unwrap(), 25);
    assert_eq!(from_sgf_coord('A').unwrap(), 26);
    assert_eq!(from_sgf_coord('Z').unwrap(), 51);
    assert!(from_sgf_coord('5').is_err());
}
