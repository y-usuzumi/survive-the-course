// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/

use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

pub trait Solution {
    fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String>;
}

pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    // There may be loops in the recipe chain
    fn resolve_recipe(
        recipe: &str,
        recipe_map: &HashMap<String, Vec<String>>,
        resolved_recipes: &mut HashSet<String>,
        unresolvable_recipes: &mut HashSet<String>,
        visited_recipes: &mut HashSet<String>,
    ) -> bool {
        if resolved_recipes.contains(recipe) {
            return true;
        }

        if unresolvable_recipes.contains(recipe) {
            return false;
        }

        if !recipe_map.contains_key(recipe) {
            return false;
        }

        if visited_recipes.contains(recipe) {
            return false;
        }

        visited_recipes.insert(recipe.into());

        for ingredient in recipe_map.get(recipe).unwrap() {
            if Self::resolve_recipe(
                ingredient,
                recipe_map,
                resolved_recipes,
                unresolvable_recipes,
                visited_recipes,
            ) {
                continue;
            }

            unresolvable_recipes.insert(recipe.into());
            return false;
        }

        resolved_recipes.insert(recipe.into());
        return true;
    }
}

impl Solution for Solution1 {
    fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut resolved_recipes: HashSet<String> = HashSet::from_iter(supplies.into_iter());
        let mut recipe_map: HashMap<String, Vec<String>> = HashMap::new();
        for (recipe, ingredient) in recipes.iter().zip(ingredients) {
            let mut necessary_ingredients = Vec::new();
            for ing in ingredient {
                if !resolved_recipes.contains(&ing) {
                    necessary_ingredients.push(ing);
                }
            }
            if necessary_ingredients.is_empty() {
                resolved_recipes.insert(recipe.into());
            } else {
                recipe_map.insert(recipe.clone(), necessary_ingredients);
            }
        }
        let mut unresolvable_recipes: HashSet<String> = HashSet::new();
        let mut visited_recipes: HashSet<String> = HashSet::new();

        let mut result = Vec::new();

        for recipe in recipes {
            if Self::resolve_recipe(
                &recipe,
                &recipe_map,
                &mut resolved_recipes,
                &mut unresolvable_recipes,
                &mut visited_recipes,
            ) {
                result.push(recipe.clone());
            }
        }

        return result;
    }
}

impl Solution for Solution2 {
    fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let recipes_backup = recipes.clone();
        let mut supplies: HashSet<String> = HashSet::from_iter(supplies);
        let mut supply_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut count_map: HashMap<String, i32> = HashMap::new();
        let mut start_recipes: HashSet<String> = HashSet::from_iter(recipes.clone());
        for (recipe, ingredient) in recipes.iter().zip(ingredients) {
            for ing in ingredient {
                if supplies.contains(&ing) {
                    continue;
                }
                supply_map
                    .entry(ing.into())
                    .or_insert(vec![])
                    .push(recipe.into());
                *count_map.entry(recipe.into()).or_insert(0) += 1;
                start_recipes.remove(recipe);
            }
        }
        let mut result = Vec::new();
        let mut q = VecDeque::from_iter(start_recipes);
        while !q.is_empty() {
            let recipe = q.pop_front().unwrap();
            if recipes_backup.contains(&recipe) {
                result.push(recipe.clone());
            }
            if supply_map.contains_key(&recipe) {
                for supplee in supply_map.get(&recipe).unwrap() {
                    *count_map.get_mut(supplee).unwrap() -= 1;
                    if count_map.get(supplee).unwrap_or(&0) == &0 {
                        q.push_back(supplee.into());
                    }
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use test_util::{assert_eq_ignore_order, strs_into_strings};

    use super::*;

    mod solution1 {
        use super::*;
        #[test]
        fn test_1() {
            let recipes = strs_into_strings(vec!["bread"]);
            let ingredients = vec![strs_into_strings(vec!["yeast", "flour"])];
            let supplies = strs_into_strings(vec!["yeast", "flour", "corn"]);
            assert_eq!(
                Solution1::find_all_recipes(recipes, ingredients, supplies),
                vec!["bread"]
            );
        }

        #[test]
        fn test_2() {
            let recipes = strs_into_strings(vec!["bread", "sandwich", "burger"]);
            let ingredients = vec![
                strs_into_strings(vec!["yeast", "flour"]),
                strs_into_strings(vec!["bread", "meat"]),
                strs_into_strings(vec!["sandwich", "meat", "bread"]),
            ];
            let supplies = strs_into_strings(vec!["yeast", "flour", "meat"]);
            assert_eq_ignore_order(
                Solution1::find_all_recipes(recipes, ingredients, supplies),
                strs_into_strings(vec!["bread", "sandwich", "burger"]),
            );
        }

        #[test]
        fn test_3() {
            let recipes = strs_into_strings(vec!["ju", "fzjnm", "x", "e", "zpmcz", "h", "q"]);
            let ingredients = vec![
                strs_into_strings(vec!["d"]),
                strs_into_strings(vec!["hveml", "f", "cpivl"]),
                strs_into_strings(vec!["cpivl", "zpmcz", "h", "e", "fzjnm", "ju"]),
                strs_into_strings(vec!["cpivl", "hveml", "zpmcz", "ju", "h"]),
                strs_into_strings(vec!["h", "fzjnm", "e", "q", "x"]),
                strs_into_strings(vec!["d", "hveml", "cpivl", "q", "zpmcz", "ju", "e", "x"]),
                strs_into_strings(vec!["f", "hveml", "cpivl"]),
            ];
            let supplies = strs_into_strings(vec!["f", "hveml", "cpivl", "d"]);
            assert_eq_ignore_order(
                Solution1::find_all_recipes(recipes, ingredients, supplies),
                strs_into_strings(vec!["ju", "fzjnm", "q"]),
            );
        }
    }

    mod solution2 {
        use super::*;
        #[test]
        fn test_1() {
            let recipes = strs_into_strings(vec!["bread"]);
            let ingredients = vec![strs_into_strings(vec!["yeast", "flour"])];
            let supplies = strs_into_strings(vec!["yeast", "flour", "corn"]);
            assert_eq!(
                Solution2::find_all_recipes(recipes, ingredients, supplies),
                vec!["bread"]
            );
        }

        #[test]
        fn test_2() {
            let recipes = strs_into_strings(vec!["bread", "sandwich", "burger"]);
            let ingredients = vec![
                strs_into_strings(vec!["yeast", "flour"]),
                strs_into_strings(vec!["bread", "meat"]),
                strs_into_strings(vec!["sandwich", "meat", "bread"]),
            ];
            let supplies = strs_into_strings(vec!["yeast", "flour", "meat"]);
            assert_eq_ignore_order(
                Solution2::find_all_recipes(recipes, ingredients, supplies),
                strs_into_strings(vec!["bread", "sandwich", "burger"]),
            );
        }

        #[test]
        fn test_3() {
            let recipes = strs_into_strings(vec!["ju", "fzjnm", "x", "e", "zpmcz", "h", "q"]);
            let ingredients = vec![
                strs_into_strings(vec!["d"]),
                strs_into_strings(vec!["hveml", "f", "cpivl"]),
                strs_into_strings(vec!["cpivl", "zpmcz", "h", "e", "fzjnm", "ju"]),
                strs_into_strings(vec!["cpivl", "hveml", "zpmcz", "ju", "h"]),
                strs_into_strings(vec!["h", "fzjnm", "e", "q", "x"]),
                strs_into_strings(vec!["d", "hveml", "cpivl", "q", "zpmcz", "ju", "e", "x"]),
                strs_into_strings(vec!["f", "hveml", "cpivl"]),
            ];
            let supplies = strs_into_strings(vec!["f", "hveml", "cpivl", "d"]);
            assert_eq_ignore_order(
                Solution2::find_all_recipes(recipes, ingredients, supplies),
                strs_into_strings(vec!["ju", "fzjnm", "q"]),
            );
        }
    }
}
