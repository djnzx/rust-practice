/// counting_valleys
fn countingValleys(steps: i32, path: &str) -> i32 {
    todo!()
}

fn countingValleys1(path: &str) -> i32 {
    let mut count = 0;
    let mut current = 0; 
    
    for c in path.chars() {
        match c {
            'U' => current += 1,
            'D' => {
                if current == 0 { count += 1; }
                current -= 1;
            },
            _ => continue,
        }
    }
    
    count
}

fn countingValleys0(path: &str) -> i32 {
    let mut count = 0;
    let mut current = 0;
    let mut prev: i32;
    
    for c in path.chars() {
        prev = current;
        match c {
            'U' => current += 1,
            'D' => current -= 1,
            _ => continue,
        }
        if current == 0 && prev == -1 {count+=1;}
    }
    
    count
}

#[test]
fn test1() {

}
