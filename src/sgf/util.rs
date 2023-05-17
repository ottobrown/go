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

#[test]
fn to_coord_test() {
    assert_eq!(to_sgf_coord(0), Ok('a'));
    assert_eq!(to_sgf_coord(25), Ok('z'));
    assert_eq!(to_sgf_coord(26), Ok('A'));
    assert_eq!(to_sgf_coord(51), Ok('Z'));
    assert_eq!(to_sgf_coord(52), Err(SgfError::CoordTooBig));
}

#[test]
fn from_coord_test() {
    assert_eq!(from_sgf_coord('a'), Ok(0));
    assert_eq!(from_sgf_coord('z'), Ok(25));
    assert_eq!(from_sgf_coord('A'), Ok(26));
    assert_eq!(from_sgf_coord('Z'), Ok(51));
    assert_eq!(from_sgf_coord('5'), Err(SgfError::InvalidCoordChar));
}
