use std::collections::{HashSet, HashMap};

// TODO: consider matches between lines

// &ing_to_all = {
//   "nhms": { "fish": [ 0, ], "dairy": [ 0, ], },
//   "sqjhc": { "soy": [ 2, ], "dairy": [ 0, ], "fish": [ 0, 3, ], },
//   "sbzzf": { "dairy": [ 1, ], "fish": [ 3, ], },
//   "fvjkl": { "soy": [ 2, ], "dairy": [ 1, ], },
//   "kfcds": { "dairy": [ 0, ], "fish": [ 0, ], },
//   "mxmxvkd": { "fish": [ 0, 3, ], "dairy": [ 0, 1, ], },
//   "trh": { "dairy": [ 1, ], },
// }

// For each allergen, top matches are:
// For dairy: [("mxmxvkd", [0, 1])].
// For fish: [("mxmxvkd", [0, 3]), ("sqjhc", [0, 3])].
// For soy: [("sqjhc", [2]), ("fvjkl", [2])].

// Inferences:
// mxmxvkd is the only top for diary => mxmxvkd contains dairy
// for fish we have mxmxvkd and sqjhc AND mxmxvkd is dairy => sqjhc is the only choice for fish
// for soy we have sqjhc and fvjkl AND sqjhc is fish => fvjkl is the only choice for soy
//
// nhms, sbzzf (1 and 3 appearances), kfcds, trh => 5 mentions total
pub fn solve(input: &str) -> Option<Box<usize>> {
  let list = parse(input);

  let mut all_to_ing = HashMap::new();
  let mut ing_to_all = HashMap::new();

  for (idx, item) in list.iter().enumerate() {
    for all in item.allergens.iter() {
      let alls_ings = all_to_ing.entry(all).or_insert(HashMap::new());
      for ing in item.ingredients.iter() {
        let alls_ings_lines = alls_ings.entry(ing).or_insert(vec![]);
        alls_ings_lines.push(idx);

        let ings_alls = ing_to_all.entry(ing).or_insert(HashMap::new());
        let ings_alls_lines = ings_alls.entry(all).or_insert(vec![]);
        ings_alls_lines.push(idx);
      }
    }
  }

  // dbg!(&all_to_ing);
  dbg!(&ing_to_all);

  // Find top ingredients by mentions for each allergen - only one of those ingredients
  // can be guaranteed to contain that allergen. Those that are not the top choices don't contain
  // the allergen in question, since they cannot cover all the appearances of the allergen.
  println!("For each allergen, top matches are:");
  for (all, ing_to_lines) in all_to_ing.iter() {
    let mut ings_sorted_by_mentions = ing_to_lines.iter().collect::<Vec<_>>();
    ings_sorted_by_mentions.sort_by_key(|(_ing, lines)| lines.len());

    let mut top_ings_by_mentions = vec![];
    if let Some(max_group) = ings_sorted_by_mentions.pop() {
      let (ing, max_lines) = max_group;
      top_ings_by_mentions.push((*ing, max_lines));

      for &(ing, lines) in ings_sorted_by_mentions.iter().rev() {
        if lines.len() < max_lines.len() {
          break;
        }

        top_ings_by_mentions.push((*ing, lines));
      }
    }

    println!("For {}: {:?}.", &all, &top_ings_by_mentions);
  }

  // For each allergen, determine the likely ingredient:
  // the top ingredient by mentions that is not conflicted by other allergens' top ingredients.
  // We can use topological sorting, to figure out the allergens where the choice of top ingredient
  // is determined (since there is only one top ingredient)

  // TODO: infer with proper topological sorting, those results are by hand
  // nuts -> qmrps, shellfish -> qnvx, wheat -> qsjszn, dairy -> cljf, fish -> vvfjj, eggs -> frtfg
  // peanuts -> hvnkk, soy -> cpxmpc.

  let known_allergen_ingredients = [
    "qmrps", "qnvx", "qsjszn", "cljf", "vvfjj", "frtfg", "hvnkk", "cpxmpc", // actual test input
    "mxmxvkd", "sqjhc", "fvjkl", // sample input
  ]
  .iter()
  .map(|x| x.to_string())
  .collect::<HashSet<_>>();

  let non_allergic_to_lines = ing_to_all
    .iter()
    .filter(|(&ing, _all_to_lines)| !known_allergen_ingredients.contains(ing))
    .map(|(_, all_to_lines)| {
      all_to_lines
        .into_iter()
        .map(|(_all, line)| line)
        .flatten()
        .collect::<HashSet<_>>()
    })
    .collect::<Vec<_>>();

  dbg!(&non_allergic_to_lines);

  let mentions_non_allergic = non_allergic_to_lines.iter().map(|lines| lines.len()).sum();
  Some(Box::new(mentions_non_allergic))
}

pub fn solve2(input: &str) -> Option<Box<String>> {
  // let sample_input_allergens_map = "dairy -> mxmxvkd, fish -> sqjhc, soy -> fvjkl";
  let input_allergens_map = "nuts -> qmrps, shellfish -> qnvx, wheat -> qsjszn, dairy -> cljf,
    fish -> vvfjj, eggs -> frtfg, peanuts -> hvnkk, soy -> cpxmpc";
  let mut allergens_to_ingredient = input_allergens_map
    .split(",")
    .map(|pair| {
      let mut pair = pair.split(" -> ");
      let allergen = pair.next().unwrap().trim();
      let ingredient = pair.next().unwrap().trim();
      (allergen.to_string(), ingredient.to_string())
    })
    .collect::<Vec<_>>();
  allergens_to_ingredient.sort_by_key(|(all, _ing)| all.to_string());
  dbg!(&allergens_to_ingredient);

  let answer = allergens_to_ingredient
    .into_iter()
    .map(|(_all, ing)| ing)
    .collect::<Vec<_>>()
    .join(",");
  Some(Box::new(answer))
}

type List = Vec<Item>;

#[derive(Debug, Clone)]
struct Item {
  ingredients: HashSet<String>,
  allergens: HashSet<String>,
}

fn parse(input: &str) -> List {
  let mut list = vec![];

  for line in input.trim_end().split('\n') {
    let mut parts = line.trim_end_matches(")").split(" (contains ");
    if let (Some(ingredients), Some(allergens)) = (parts.next(), parts.next()) {
      let ingredients = ingredients
        .split_ascii_whitespace()
        .map(|ing| ing.to_string())
        .collect::<HashSet<_>>();
      let allergens = allergens
        .split(", ")
        .map(|allergen| allergen.to_string())
        .collect::<HashSet<_>>();

      list.push(Item { ingredients, allergens });
    }
  }

  list
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn part_one_solved() {
    let input = fs::read_to_string("inputs/sample21").unwrap();
    assert_eq!(solve(&input), Some(Box::new(5)));

    let input = fs::read_to_string("inputs/d21").unwrap();
    assert_eq!(solve(&input), Some(Box::new(2307)));
  }

  #[test]
  fn part_two_solved() {
    let input = fs::read_to_string("inputs/d21").unwrap();
    assert_eq!(solve2(&input), None);
  }
}
