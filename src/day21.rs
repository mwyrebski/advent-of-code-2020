use std::collections::*;

type Foods = Vec<Food>;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Ingridient(String);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Allergen(String);

#[derive(PartialEq, Eq, Hash, Debug)]
struct Food(Vec<Ingridient>, Vec<Allergen>);

fn parse(input: &str) -> Foods {
    let contains_len = "contains ".len();
    input
        .lines()
        .map(|line| {
            let sep = line.find('(').unwrap();
            let ingridients_str = &line[..sep - 1];
            let allergens_str = {
                let tmp = &line[sep..];
                &tmp[1 + contains_len..tmp.len() - 1]
            };
            let ingridients = ingridients_str
                .split(' ')
                .map(|x| Ingridient(x.to_string()))
                .collect();
            let allergens = allergens_str
                .split(", ")
                .map(|x| Allergen(x.to_string()))
                .collect();
            Food(ingridients, allergens)
        })
        .collect()
}

fn part1(foods: &Foods) -> usize {
    let mut potentially_allergic_map = HashMap::<&Allergen, HashMap<&Ingridient, u32>>::new();
    let mut all_ingridients = Vec::new();
    let mut guessed = Vec::new();
    for Food(ingridients, allergens) in foods {
        for ingridient in ingridients {
            all_ingridients.push(ingridient);
            ////eprintln!("  check {:?}", ingridient);

            // if guessed.contains(&ingridient) {
            //     continue;
            // }

            for allergen in allergens {
                ////eprintln!("{:?}", allergen);
                let ingridients_freq = potentially_allergic_map.entry(allergen).or_default();
                let count = ingridients_freq.entry(ingridient).or_default();
                *count += 1;
                // if e.contains(&ingridient) {
                //     eprintln!("  CONTAINED");

                //     e.retain(|&x| x == ingridient);
                //     guessed.push(ingridient);
                // } else {
                //     eprintln!("  push");
                //     e.push(ingridient);
                // }
            }
        }
    }

    //eprintln!("map {:#?}", potentially_allergic_map);

    //return 0;

    loop {
        let matching = potentially_allergic_map.iter().find_map(|(k, v)| {
            let freq_map = v
                .iter()
                .filter(|(ing, _)| !guessed.contains(*ing))
                .map(|(k, v)| (*k, *v))
                .collect::<HashMap<&Ingridient, u32>>();
            let max_freq = freq_map.values().max().unwrap();

            let ingridients_with_max_freq = freq_map
                .iter()
                .filter_map(|(ingridient, count)| {
                    if count == max_freq {
                        Some(*ingridient)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&Ingridient>>();

            if ingridients_with_max_freq.len() == 1 {
                let allergen = *k;
                Some((allergen, ingridients_with_max_freq[0]))
            } else {
                None
            }
        });

        if let Some((allergen, ingridient)) = matching {
            potentially_allergic_map.remove(allergen);
            guessed.push(ingridient);
        }

        //for (&allergen, freq_map) in potentially_allergic_map.iter() {}

        if potentially_allergic_map.is_empty() {
            break;
        }
    }

    // let mut possibly_with_allergens = HashSet::new();
    // for &ingridient in potentially_allergic_map.values().flatten() {
    //     possibly_with_allergens.insert(ingridient);
    // }

    //eprintln!("all  {:#?}", all_ingridients);
    // eprintln!("possibly {:#?}", possibly_with_allergens);
    // eprintln!("guessed {:#?}", guessed);

    all_ingridients
        .iter()
        .filter(|i| !guessed.contains(i))
        .count()

    //all_ingridients.difference(&guessed).count()
    //0
}

fn part2(foods: &Foods) -> String {
    let mut potentially_allergic_map = HashMap::<&Allergen, HashMap<&Ingridient, u32>>::new();
    let mut all_ingridients = Vec::new();
    for Food(ingridients, allergens) in foods {
        for ingridient in ingridients {
            all_ingridients.push(ingridient);
            for allergen in allergens {
                let ingridients_freq = potentially_allergic_map.entry(allergen).or_default();
                let count = ingridients_freq.entry(ingridient).or_default();
                *count += 1;
            }
        }
    }

    let mut guessed_map = HashMap::<&Ingridient, &Allergen>::new();
    loop {
        let matching = potentially_allergic_map.iter().find_map(|(k, v)| {
            let freq_map = v
                .iter()
                .filter(|(ing, _)| !guessed_map.contains_key(*ing))
                .map(|(k, v)| (*k, *v))
                .collect::<HashMap<&Ingridient, u32>>();
            let max_freq = freq_map.values().max().unwrap();

            let ingridients_with_max_freq = freq_map
                .iter()
                .filter_map(|(ingridient, count)| {
                    if count == max_freq {
                        Some(*ingridient)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&Ingridient>>();

            if ingridients_with_max_freq.len() == 1 {
                let allergen = *k;
                Some((ingridients_with_max_freq[0], allergen))
            } else {
                None
            }
        });

        if let Some((ingridient, allergen)) = matching {
            potentially_allergic_map.remove(allergen);
            guessed_map.insert(ingridient, allergen);
        }

        if potentially_allergic_map.is_empty() {
            break;
        }
    }

    let mut guessed: Vec<_> = guessed_map.into_iter().collect();
    guessed.sort_by_key(|(_, allergen)| *allergen);
    let values = guessed
        .iter()
        .map(|(ingridient, _)| {
            let Ingridient(s) = ingridient;
            s.clone()
        })
        .collect::<Vec<_>>()
        .join(",");

    values
    // let mut possibly_with_allergens = HashSet::new();
    // for &ingridient in potentially_allergic_map.values().flatten() {
    //     possibly_with_allergens.insert(ingridient);
    // }

    //eprintln!("all  {:#?}", all_ingridients);
    // eprintln!("possibly {:#?}", possibly_with_allergens);
    // eprintln!("guessed {:#?}", guessed);
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
                    Ingridient("abc".to_string()),
                    Ingridient("def".to_string()),
                    Ingridient("ghi".to_string())
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
