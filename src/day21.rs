use std::collections::*;

type Foods = Vec<Food>;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Ingredient(String);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Allergen(String);

#[derive(PartialEq, Eq, Hash, Debug)]
struct Food(Vec<Ingredient>, Vec<Allergen>);

fn parse(input: &str) -> Foods {
    let contains_len = "contains ".len();
    input
        .lines()
        .map(|line| {
            let sep = line.find('(').unwrap();
            let ingredients_str = &line[..sep - 1];
            let allergens_str = {
                let tmp = &line[sep..];
                &tmp[1 + contains_len..tmp.len() - 1]
            };
            let ingredients = ingredients_str
                .split(' ')
                .map(|x| Ingredient(x.to_string()))
                .collect();
            let allergens = allergens_str
                .split(", ")
                .map(|x| Allergen(x.to_string()))
                .collect();
            Food(ingredients, allergens)
        })
        .collect()
}

fn find_matches(foods: &Foods) -> HashMap<&Ingredient, &Allergen> {
    let mut potentially_allergic_ingredients_map =
        HashMap::<&Allergen, HashMap<&Ingredient, u32>>::new();
    for Food(ingredients, allergens) in foods {
        for ingredient in ingredients {
            for allergen in allergens {
                let ingredients_freq = potentially_allergic_ingredients_map
                    .entry(allergen)
                    .or_default();
                let count = ingredients_freq.entry(ingredient).or_default();
                *count += 1;
            }
        }
    }

    let mut match_map = HashMap::<&Ingredient, &Allergen>::new();

    while !potentially_allergic_ingredients_map.is_empty() {
        let matching_tuple_opt =
            potentially_allergic_ingredients_map
                .iter()
                .find_map(|(allergen, freq_map)| {
                    let freq_map_except_already_guessed = freq_map
                        .iter()
                        .filter(|(ing, _)| !match_map.contains_key(*ing))
                        .map(|(k, v)| (*k, *v))
                        .collect::<HashMap<&Ingredient, u32>>();

                    let max_freq = freq_map_except_already_guessed.values().max().unwrap();

                    let most_freq_ingredients = freq_map_except_already_guessed
                        .iter()
                        .filter_map(|(ingredient, count)| {
                            if count == max_freq {
                                Some(*ingredient)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<&Ingredient>>();

                    match most_freq_ingredients.as_slice() {
                        [ingredient] => Some((*ingredient, *allergen)),
                        _ => None,
                    }
                });

        if let Some((ingredient, allergen)) = matching_tuple_opt {
            potentially_allergic_ingredients_map.remove(allergen);
            match_map.insert(ingredient, allergen);
        }
    }

    match_map
}

fn part1(foods: &Foods) -> usize {
    let matches = find_matches(foods);

    let all_ingredients: Vec<&Ingredient> = foods
        .iter()
        .map(|Food(ingredients, _)| ingredients)
        .flatten()
        .collect();

    all_ingredients
        .into_iter()
        .filter(|i| !matches.contains_key(i))
        .count()
}

fn part2(foods: &Foods) -> String {
    let matches = find_matches(foods);

    let mut ingredient_allergen_vec: Vec<_> = matches.into_iter().collect();
    ingredient_allergen_vec.sort_by_key(|(_, allergen)| *allergen);
    ingredient_allergen_vec
        .iter()
        .map(|(Ingredient(ingredient_string), _)| ingredient_string.clone())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn run() {
    let input = include_str!("input/day21.txt");
    let parsed = &parse(input);
    println!("Day 21/1: {}", part1(parsed));
    println!("Day 21/2: {}", part2(parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE1: &'static str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    #[test]
    fn test_day21_parse() {
        assert_eq!(
            vec![Food(
                vec![
                    Ingredient("abc".to_string()),
                    Ingredient("def".to_string()),
                    Ingredient("ghi".to_string())
                ],
                vec![Allergen("asdf".to_string()), Allergen("qwer".to_string())]
            )],
            parse("abc def ghi (contains asdf, qwer)")
        );
    }

    #[test]
    fn test_day21_part1_sample1() {
        assert_eq!(5, part1(&parse(SAMPLE1)));
    }

    #[test]
    fn test_day21_part2_sample1() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl", part2(&parse(SAMPLE1)));
    }
}
