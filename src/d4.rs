use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<i64> {
  let valid_passports = parse(input)
    .iter()
    .filter(|passport| is_valid(passport))
    .count();

  Some(valid_passports as i64)
}

pub fn solve2(input: &str) -> Option<i64> {
  let valid_passports = parse(input)
    .iter()
    .filter(|passport| is_valid2(passport))
    .count();

  Some(valid_passports as i64)
}

fn parse(input: &str) -> Vec<HashMap<String, String>> {
  input
    .split("\n\n")
    .map(|passport| {
      passport
        .split_ascii_whitespace()
        .map(|kv| {
          let mut kv = kv.split(':');
          let key = kv.next().unwrap().to_string();
          let value = kv.next().unwrap().to_string();
          (key, value)
        })
        .collect::<HashMap<_, _>>()
    })
    .collect::<Vec<_>>()
}

fn is_valid(password: &HashMap<String, String>) -> bool {
  let keys = password.keys().collect::<HashSet<_>>();

  let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  required
    .iter()
    .all(|field| keys.contains(&field.to_string()))
}

fn is_valid2(passport: &HashMap<String, String>) -> bool {
  if is_valid(passport) {
    // we know that all fields are present, since `is_valid` checks it
    let valid_byr = is_valid_year(passport.get("byr").unwrap(), 1920, 2002);
    let valid_iyr = is_valid_year(passport.get("iyr").unwrap(), 2010, 2020);
    let valid_eyr = is_valid_year(passport.get("eyr").unwrap(), 2020, 2030);

    let valid_hgt = is_valid_height(passport.get("hgt").unwrap());
    let valid_hcl = is_valid_hcl(passport.get("hcl").unwrap());
    let valid_ecl = is_valid_ecl(passport.get("ecl").unwrap());
    let valid_pid = is_valid_pid(passport.get("pid").unwrap());

    valid_byr
      && valid_iyr
      && valid_eyr
      && valid_hgt
      && valid_hcl
      && valid_ecl
      && valid_pid
  } else {
    false
  }
}

fn is_valid_year(year: &str, min: u32, max: u32) -> bool {
  match year.parse::<u32>() {
    Ok(year) => year >= min && year <= max,
    Err(_) => false,
  }
}

fn is_valid_height(height: &str) -> bool {
  lazy_static! {
    static ref RE: Regex =
      Regex::new(r"(?P<number>\d+)(?P<unit>cm|in)").unwrap();
  }

  match RE.captures_iter(height).next() {
    Some(capture) => match (capture.name("number"), capture.name("unit")) {
      (Some(number), Some(unit)) => match number.as_str().parse::<u32>() {
        Ok(number) => match unit.as_str() {
          "cm" => number >= 150 && number <= 193,
          "in" => number >= 59 && number <= 76,
          _ => false,
        },
        Err(_) => false,
      },
      _ => false,
    },
    None => false,
  }
}

fn is_valid_hcl(hcl: &str) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  }

  RE.is_match(hcl)
}

fn is_valid_ecl(ecl: &str) -> bool {
  lazy_static! {
    static ref VALID_ECLS: Vec<&'static str> =
      vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
  }

  VALID_ECLS.contains(&ecl)
}

fn is_valid_pid(pid: &str) -> bool {
  pid.len() == 9 && pid.chars().all(|ch| ch.is_ascii_digit())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn parser_works() {
    let input = fs::read_to_string("inputs/sample4").unwrap();

    let passport = parse(&input);
    assert_eq!(passport.len(), 4);

    assert_eq!(passport[0].keys().len(), 8);
    assert_eq!(passport[0].get("ecl"), Some(&"gry".to_string()));
    assert_eq!(passport[0].get("pid"), Some(&"860033327".to_string()));
    assert_eq!(passport[0].get("eyr"), Some(&"2020".to_string()));
    assert_eq!(passport[0].get("hcl"), Some(&"#fffffd".to_string()));
    assert_eq!(passport[0].get("byr"), Some(&"1937".to_string()));
    assert_eq!(passport[0].get("iyr"), Some(&"2017".to_string()));
    assert_eq!(passport[0].get("cid"), Some(&"147".to_string()));
    assert_eq!(passport[0].get("hgt"), Some(&"183cm".to_string()));

    assert_eq!(passport[1].keys().len(), 7);
    assert_eq!(passport[1].get("pid"), Some(&"028048884".to_string()));
    assert_eq!(passport[1].get("hcl"), Some(&"#cfa07d".to_string()));

    assert_eq!(passport[2].keys().len(), 7);
    assert_eq!(passport[2].get("eyr"), Some(&"2024".to_string()));

    assert_eq!(passport[3].keys().len(), 6);
    assert_eq!(passport[3].get("hcl"), Some(&"#cfa07d".to_string()));
    assert_eq!(passport[3].get("hgt"), Some(&"59in".to_string()));
  }

  #[test]
  fn part_one_works_on_sample() {
    let input = fs::read_to_string("inputs/sample4").unwrap();
    assert_eq!(solve(&input), Some(2));
  }

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/d4").unwrap();
    assert_eq!(solve(&input), Some(245));
  }

  #[test]
  fn validators_work() {
    assert!(is_valid_year("2002", 1920, 2002));

    assert!(!is_valid_year("2003", 1920, 2002));

    assert!(is_valid_height("60in"));
    assert!(is_valid_height("190cm"));

    assert!(!is_valid_height("190in"));
    assert!(!is_valid_height("190"));

    assert!(is_valid_hcl("#123abc"));

    assert!(!is_valid_hcl("#123abz"));
    assert!(!is_valid_hcl("123abc"));

    assert!(is_valid_ecl("brn"));
    assert!(!is_valid_ecl("wat"));

    assert!(is_valid_pid("000000001"));
    assert!(!is_valid_pid("0123456789"));
  }

  #[test]
  fn valid2_works() {
    let passport = parse(
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
    )
    .into_iter()
    .next()
    .unwrap();

    assert!(is_valid2(&passport));
  }

  #[test]
  fn part_two_works_on_sample() {
    let valid_passports = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

    assert_eq!(parse(valid_passports).len(), 4);
    assert_eq!(solve2(&valid_passports), Some(4));

    let invalid_passports = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

    assert_eq!(parse(invalid_passports).len(), 4);
    assert_eq!(solve2(invalid_passports), Some(0));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d4").unwrap();
    assert_eq!(solve2(&input), Some(133));
  }
}
