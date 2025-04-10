const LEVELS: usize = 4; // Кількість трикутників

fn main() {
    let mut result = String::new();

    (0..LEVELS).for_each(|level| {
        (0..=level + 2).for_each(|i| {
            let width = (LEVELS + 2) * 2 - 1;
            let stars = 1 + i * 2;
            let spaces = (width - stars) / 2;
            result += &" ".repeat(spaces);
            result += &"*".repeat(stars);
            result += "\n";
        });
    });

    // Стовбур
    let trunk_width = 3;
    let trunk_height = 2;
    let total_width = (LEVELS + 2) * 2 - 1;
    let padding = (total_width - trunk_width) / 2;

    (0..trunk_height_
