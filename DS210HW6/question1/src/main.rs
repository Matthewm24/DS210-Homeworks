use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

fn strings_from_data(
    name: &str,
    recipe: &str,
    people_categories_data: &str,
    categories_ingredients_data: &str,
    recipes_data: &str,
) -> String {
    let mut output = String::new();

    let mut people_categories = HashMap::new();
    for line in people_categories_data.lines() {
        if let Some((person, categories)) = line.split_once(": ") {
            let category_set: HashSet<_> = categories
                .split(',')
                .map(str::trim)
                .map(|s| s.to_lowercase())
                .collect();
            people_categories.insert(person.to_lowercase(), category_set);
        }
    }

    let mut category_ingredients = HashMap::new();
    let mut ingredient_to_category = HashMap::new();
    for line in categories_ingredients_data.lines() {
        if let Some((category, ingredients)) = line.split_once(": ") {
            let ingredient_set: HashSet<_> = ingredients
                .split(',')
                .map(str::trim)
                .map(|s| s.to_lowercase())
                .collect();
            for ingredient in &ingredient_set {
                ingredient_to_category.insert(ingredient.clone(), category.to_lowercase());
            }
            category_ingredients.insert(category.to_lowercase(), ingredient_set);
        }
    }

    let mut recipe_ingredients = HashMap::new();
    for line in recipes_data.lines() {
        if let Some((recipe_name, ingredients)) = line.split_once(": ") {
            let ingredient_list: Vec<_> = ingredients
                .split(',')
                .map(str::trim)
                .map(|s| s.to_lowercase())
                .collect();
            recipe_ingredients.insert(recipe_name.to_lowercase(), ingredient_list);
        }
    }

    if name == "popular recipes" {
        let mut recipe_scores: Vec<(&String, usize)> = recipe_ingredients
            .iter()
            .map(|(recipe_name, ingredients)| {
                let mut count = 0;
                for (_person, categories) in &people_categories {
                    let liked = ingredients
                        .iter()
                        .filter(|&ingredient| {
                            if let Some(category) = ingredient_to_category.get(ingredient) {
                                categories.contains(category)
                            } else {
                                false
                            }
                        })
                        .count();
                    let threshold = (0.6 * ingredients.len() as f64).ceil() as usize;
                    if liked >= threshold {
                        count += 1;
                    }
                }
                (recipe_name, count)
            })
            .collect();

        recipe_scores.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
        });

        output.push_str("Top 3 popular recipes:\n");
        for (recipe, count) in recipe_scores.iter().take(3) {
            output.push_str(&format!("{} (liked by {} people)\n", recipe, count));
        }
        return output;
    }

    if !people_categories.contains_key(name) {
        return format!("Person '{}' not found.", name);
    }

    if !recipe_ingredients.contains_key(recipe) {
        return format!("Recipe '{}' not found.", recipe);
    }

    let user_categories = match people_categories.get(name) {
        Some(c) => c,
        None => return format!("Person '{}' not found.", name),
    };

    let ingredients = match recipe_ingredients.get(recipe) {
        Some(i) => i,
        None => return format!("Recipe '{}' not found.", recipe),
    };

    let total = ingredients.len();
    let liked = ingredients
        .iter()
        .filter(|&ingredient| {
            if let Some(category) = ingredient_to_category.get(ingredient) {
                user_categories.contains(category)
            } else {
                false
            }
        })
        .count();

    let threshold = (0.6 * total as f64).ceil() as usize;
    if liked >= threshold {
        output.push_str(&format!("{} likes the recipe '{}'.", name, recipe));
    } else {
        output.push_str(&format!("{} does NOT like the recipe '{}'.", name, recipe));
    }

    output
}
fn main() {
    let mut name = String::new();
    println!("name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim().to_lowercase();

    let mut recipe = String::new();
    println!("recipe:");
    io::stdin()
        .read_line(&mut recipe)
        .expect("Failed to read recipe");
    let recipe = recipe.trim().to_lowercase();

    let people_categories_path = "../hw_06_new/people_categories.txt";
    let categories_ingredients_path = "../hw_06_new/categories_ingredients.txt";
    let recipes_path = "../hw_06_new/recipes.txt";

    let people_categories_data =
        fs::read_to_string(people_categories_path).expect("Failed to read people categories file");
    let categories_ingredients_data = fs::read_to_string(categories_ingredients_path)
        .expect("Failed to read categories ingredients file");
    let recipes_data = fs::read_to_string(recipes_path).expect("Failed to read recipes file");

    let result = strings_from_data(
        &name,
        &recipe,
        &people_categories_data,
        &categories_ingredients_data,
        &recipes_data,
    );
    print!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popular_recipes() {
        let people_categories_data = include_str!("../../hw_06_new/people_categories.txt");
        let categories_ingredients_data =
            include_str!("../../hw_06_new/categories_ingredients.txt");
        let recipes_data = include_str!("../../hw_06_new/recipes.txt");

        let result = strings_from_data(
            "popular recipes",
            "",
            people_categories_data,
            categories_ingredients_data,
            recipes_data,
        );

        assert!(result.contains("Top 3 popular recipes:"));
    }

    #[test]
    fn test_unknown_person() {
        let people_categories_data = include_str!("../../hw_06_new/people_categories.txt");
        let categories_ingredients_data =
            include_str!("../../hw_06_new/categories_ingredients.txt");
        let recipes_data = include_str!("../../hw_06_new/recipes.txt");

        let result = strings_from_data(
            "unknown_person",
            "recipe0",
            people_categories_data,
            categories_ingredients_data,
            recipes_data,
        );

        assert_eq!(result.trim(), "Person 'unknown_person' not found.");
    }
}
