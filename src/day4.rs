extern crate nom;

use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::multi::*;
use nom::IResult;
use std::fs;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq)]
struct PassportField<'a> {
    fname: &'a str,
    fdata: &'a str,
}

pub fn day4() {
    let f = fs::read_to_string("inputs/day4.txt").expect("file not found");
    let (_, ps) = raw_passports(&f).expect("bad parse ");
    let check = |p: RawPassport| p.len() == 8 || p.len() == 7 && !field_names(&p).contains("cid");
    let valid_passports = ps
        .iter()
        .fold(0, |acc, p| if check(p.clone()) { acc + 1 } else { acc });
    println!("day 4 part 1: {}", valid_passports);
    let valid_passports_2 = count_valid(&ps);
    println!("day 4 part 2: {}", valid_passports_2);
}

fn count_valid<'a>(ps: &'a Vec<RawPassport<'a>>) -> usize {
    ps.iter()
        .map(|p| check_raw(p))
        .filter(|v| v.is_some())
        .count()
}

type RawPassport<'a> = HashMap<&'a str, &'a str>;

fn to_year(s: &str) -> Option<usize> {
    if s.len() != 4 {
        None
    } else {
        s.parse().ok()
    }
}

fn coerce(b: bool) -> Option<()> {
    match b {
        true => Some(()),
        false => None,
    }
}

enum Height {
    Cm(usize),
    In(usize),
}
use Height::*;

fn parse_height(i: &str) -> IResult<&str, Height> {
    let (rest, nstr) = digit1(i)?;
    let (rest, unit) = alt((tag("in"), tag("cm")))(rest)?;
    let n = nstr.parse().unwrap();
    match (rest, unit) {
        ("", "cm") => Ok((rest, Cm(n))),
        ("", "in") => Ok((rest, In(n))),
        _ => panic!("can't happen"),
    }
}

// A really janky and horrible way to short circuit boolean checks to
// return false. There's probably a cleaner way to do it...perhaps the
// anyhow crate? I miss Haskell :(
fn check_raw<'a>(pp: &'a RawPassport) -> Option<()> {
    // all fields except CID must be present
    coerce(pp.len() == 8 || (pp.len() == 7 && !field_names(&pp).contains("cid")))?;

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr = to_year(pp["byr"])?;
    coerce(byr >= 1920 && byr <= 2002)?;

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr = to_year(pp["iyr"])?;
    coerce(iyr >= 2010 && iyr <= 2020)?;

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr = to_year(pp["eyr"])?;
    coerce(eyr >= 2020 && eyr <= 2030)?;

    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    let hgt = match parse_height(pp["hgt"]) {
        Ok((_, height)) => Some(height),
        Err(_) => None,
    }?;

    match hgt {
        Cm(n) => coerce(n >= 150 && n <= 193),
        In(n) => coerce(n >= 59 && n <= 76),
    }?;

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let hcl = pp["hcl"];
    let fst_char = hcl.chars().nth(0)?;
    coerce(fst_char == '#')?;
    let mut hex_chars = hcl.chars();
    hex_chars.next();
    coerce(hex_chars.all(|c| c.is_alphanumeric()))?;

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    coerce(colors.iter().any(|c| *c == pp["ecl"]))?;

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid = pp["pid"];
    coerce(pid.chars().count() == 9)?;
    coerce(pid.chars().all(|c| c.is_digit(10)))?;
    Some(())
    // }
}

// Parsing Code

fn passport_field(i: &str) -> IResult<&str, PassportField> {
    let (rest, fname) = alphanumeric1(i)?;
    let (rest, _) = tag(":")(rest)?;
    // someday I will learn nom better ... today is not that day
    let (rest, fdata) = is_a("qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890#")(rest)?;
    Ok((rest, PassportField { fname, fdata }))
}

fn sep(i: &str) -> IResult<&str, &str> {
    alt((tag("\n"), tag(" ")))(i)
}


fn raw_passport(i: &str) -> IResult<&str, RawPassport> {
    let mut m = HashMap::new();
    let (rest, fields) = separated_list1(sep, passport_field)(i)?;
    for f in fields {
        m.insert(f.fname, f.fdata);
    }
    Ok((rest, m))
}

fn raw_passports(i: &str) -> IResult<&str, Vec<RawPassport>> {
    separated_list1(tag("\n\n"), raw_passport)(i)
}

fn field_names<'a>(pp: &'a RawPassport) -> HashSet<&'a str> {
    let mut s = HashSet::new();
    for (k, _) in pp {
        s.insert(*k);
    }
    s
}

#[cfg(test)]
mod test {
    use super::*;
    // static HGT: PassportField = PassportField {
    //     fname: "hgt",
    //     fdata: "183cm",
    // };
    // static HCL: PassportField = PassportField {
    //     fname: "hcl",
    //     fdata: "#fffffd",
    // };
    // static IYR: PassportField = PassportField {
    //     fname: "iyr",
    //     fdata: "2013",
    // };

    #[test]
    fn test_passport_fields() {
        assert_eq!(
            passport_field("ecl:gry"),
            Ok((
                "",
                PassportField {
                    fname: "ecl",
                    fdata: "gry"
                }
            ))
        );
        assert_eq!(
            passport_field("hcl:#fffffd"),
            Ok((
                "",
                PassportField {
                    fname: "hcl",
                    fdata: "#fffffd"
                }
            ))
        );
        assert_eq!(
            passport_field("hgt:183cm"),
            Ok((
                "",
                PassportField {
                    fname: "hgt",
                    fdata: "183cm"
                }
            ))
        );
    }

    //     #[test]
    //     fn test_passport() {
    //         let s = "iyr:2013 hgt:183cm
    // hcl:#fffffd";
    //         assert_eq!(passport(s), Ok(("", vec![iyr, hgt, hcl])));
    //     }

    #[test]
    fn test_input() {
        let f = fs::read_to_string("src/testInput.txt").expect("file not found");
        let ps = raw_passports(&f);
        assert!(ps.is_ok());
        println!("{:?}", ps)
    }

    #[test]
    fn test_valid() {
        let s = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";
        let (_, ps) = raw_passports(&s).ok().unwrap();
        assert_eq!(count_valid(&ps), 3);
    }
}
