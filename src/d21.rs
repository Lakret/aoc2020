use std::collections::{HashSet, HashMap};

type AllergenToIngredientWithLines = HashMap<String, HashMap<String, Vec<usize>>>;
type IngredientToAllergenWithLines = HashMap<String, HashMap<String, Vec<usize>>>;

pub fn solve(input: &str) -> Option<Box<usize>> {
  let (allergen_to_ingredient_with_lines, ingredient_to_allergen_with_lines) = parse(input);
  let allergen_to_ingredient = get_allergen_to_ingredient(&allergen_to_ingredient_with_lines);

  let known_allergen_ingredients = allergen_to_ingredient
    .iter()
    .map(|(_, ingredient)| ingredient)
    .collect::<HashSet<_>>();

  let non_allergic_to_lines = ingredient_to_allergen_with_lines
    .iter()
    .filter(|(ingredient, _)| !known_allergen_ingredients.contains(ingredient))
    .map(|(_, allergens_to_lines)| {
      allergens_to_lines
        .into_iter()
        .map(|(_all, line)| line)
        .flatten()
        .collect::<HashSet<_>>()
    })
    .collect::<Vec<_>>();

  let mentions_non_allergic = non_allergic_to_lines.iter().map(|lines| lines.len()).sum();
  Some(Box::new(mentions_non_allergic))
}

pub fn solve2(input: &str) -> Option<Box<String>> {
  let (allergen_to_ingredient_with_lines, _) = parse(input);
  let mut allergen_to_ingredient = get_allergen_to_ingredient(&allergen_to_ingredient_with_lines)
    .into_iter()
    .collect::<Vec<_>>();
  allergen_to_ingredient.sort_by_key(|(all, _ing)| all.to_string());

  let answer = allergen_to_ingredient
    .into_iter()
    .map(|(_all, ing)| ing)
    .collect::<Vec<_>>()
    .join(",");
  Some(Box::new(answer))
}

/// For each allergen, determine the likely ingredient:
/// the top ingredient by mentions that is not conflicted by other allergens' top ingredients.
/// We can use topological sorting-like algorithm, to figure out the allergens where the choice of top ingredient
/// is determined (since there is only one top ingredient).
fn get_allergen_to_ingredient(
  allergen_to_ingredient_with_lines: &AllergenToIngredientWithLines,
) -> HashMap<String, String> {
  let mut allergen_to_top_ingredients_by_mentions =
    get_allergen_to_top_ingredients_by_mentions(allergen_to_ingredient_with_lines);

  let mut allergen_to_ingredient = HashMap::new();
  get_allergen_to_ingredient_inner(
    &mut allergen_to_top_ingredients_by_mentions,
    &mut allergen_to_ingredient,
  );

  allergen_to_ingredient
}

fn get_allergen_to_ingredient_inner(
  allergen_to_top_ingredients_by_mentions: &mut HashMap<String, HashSet<String>>,
  allergen_to_ingredient: &mut HashMap<String, String>,
) {
  let only_choice = allergen_to_top_ingredients_by_mentions
    .iter()
    .find(|(_all, candidate_ingredients)| candidate_ingredients.len() == 1)
    .map(|(allergen, one_ingredient_set)| {
      let ingredient = one_ingredient_set.into_iter().take(1).next().unwrap();
      (allergen.to_string(), ingredient.to_string())
    });

  if let Some((allergen, ingredient)) = only_choice {
    for (_all, candidate_ingredients) in allergen_to_top_ingredients_by_mentions.iter_mut() {
      candidate_ingredients.remove(&ingredient);
    }
    allergen_to_top_ingredients_by_mentions.remove(&allergen);

    allergen_to_ingredient.insert(allergen, ingredient);
    get_allergen_to_ingredient_inner(allergen_to_top_ingredients_by_mentions, allergen_to_ingredient);
  }
}

/// Find top ingredients by mentions for each allergen - only one of those ingredients
/// can be guaranteed to contain that allergen. Those that are not the top choices don't contain
/// the allergen in question, since they cannot cover all the appearances of the allergen.
///
/// Returns a map from allergen to a hash set of possible ingredients containing it.
fn get_allergen_to_top_ingredients_by_mentions(
  allergen_to_ingredient_with_lines: &AllergenToIngredientWithLines,
) -> HashMap<String, HashSet<String>> {
  let mut allergen_to_top_ingredients_by_mentions = HashMap::new();

  // this is essentially
  // allergen_to_ing_with_lines |> sort by lines count desc |> group by lines count |> take the first group
  for (all, ing_to_lines) in allergen_to_ingredient_with_lines.iter() {
    let mut ings_sorted_by_mentions = ing_to_lines.iter().collect::<Vec<_>>();
    ings_sorted_by_mentions.sort_by_key(|(_ing, lines)| lines.len());

    let mut top_ings_to_mentions = vec![];
    if let Some(max_group) = ings_sorted_by_mentions.pop() {
      let (ing, max_lines) = max_group;
      top_ings_to_mentions.push((ing, max_lines));

      for &(ing, lines) in ings_sorted_by_mentions.iter().rev() {
        if lines.len() < max_lines.len() {
          break;
        }

        top_ings_to_mentions.push((ing, lines));
      }
    }

    let top_ings = top_ings_to_mentions
      .into_iter()
      .map(|(ing, _mentions)| ing.to_string())
      .collect::<HashSet<_>>();

    allergen_to_top_ingredients_by_mentions.insert(all.to_string(), top_ings);
  }

  allergen_to_top_ingredients_by_mentions
}

#[derive(Debug, Clone)]
struct Item {
  ingredients: HashSet<String>,
  allergens: HashSet<String>,
}

fn parse(input: &str) -> (AllergenToIngredientWithLines, IngredientToAllergenWithLines) {
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

  let mut allergen_to_ingredient_with_lines = HashMap::new();
  let mut ingredient_to_allergen_with_lines = HashMap::new();

  for (idx, item) in list.iter().enumerate() {
    for allergen in item.allergens.iter() {
      let alls_ings = allergen_to_ingredient_with_lines
        .entry(allergen.to_string())
        .or_insert(HashMap::new());
      for ingredient in item.ingredients.iter() {
        let alls_ings_lines = alls_ings.entry(ingredient.to_string()).or_insert(vec![]);
        alls_ings_lines.push(idx);

        let ings_alls = ingredient_to_allergen_with_lines
          .entry(ingredient.to_string())
          .or_insert(HashMap::new());
        let ings_alls_lines = ings_alls.entry(allergen.to_string()).or_insert(vec![]);
        ings_alls_lines.push(idx);
      }
    }
  }

  (allergen_to_ingredient_with_lines, ingredient_to_allergen_with_lines)
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

    let solution = "cljf,frtfg,vvfjj,qmrps,hvnkk,qnvx,cpxmpc,qsjszn";
    assert_eq!(solve2(&input), Some(Box::new(solution.to_string())));
  }
}
