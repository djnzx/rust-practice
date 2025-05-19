use std::collections::HashMap;

/// char-indexes
fn char_positions(text: &str) -> HashMap<char, Vec<usize>> {
    let mut outcome: HashMap<char, Vec<usize>> = HashMap::new();

    let mut idx = 0;
    
    for c in text.chars() {
        idx += 1;
        let c = c.to_ascii_lowercase();
        if !c.is_alphabetic() { continue; }; 
        let found = outcome.contains_key(&c);
        if found {
            let indexes = outcome.get_mut(&c).unwrap();
            indexes.push(idx);
            // add index
        } else { 
            let indexes = vec![idx];
            outcome.insert(c, indexes);
        }
    }
    
    outcome
}



#[test]
fn test1() {
    let x = char_positions("Hello, world!");
    println!("{:?}", x);
}
