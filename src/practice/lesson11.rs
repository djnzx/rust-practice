/// counting_valleys
fn countValleys1(s: String) -> i32 {
    let mut count = 0;
    let mut current = 0;

    for c in s.chars() {
        match c {
            'U' => current += 1,
            'D' => {
                if current == 0 {
                    count += 1;
                }
                current -= 1;
            }
            _ => continue,
        }
    }

    count
}

fn countValleys2(s: String) -> i32 {
    let mut count = 0;
    let mut current = 0;

    for c in s.chars() {
        match c {
            'U' => {
                if current == -1 {
                    count += 1;
                }
                current += 1;
            }
            'D' => current -= 1,
            _ => continue,
        }
    }

    count
}

fn countValleys3(s: String) -> i32 {
    let mut count = 0;
    let mut current = 0;
    let mut prev: i32;

    for c in s.chars() {
        prev = current;
        match c {
            'U' => current += 1,
            'D' => current -= 1,
            _ => continue,
        }
        if current == 0 && prev == -1 {
            count += 1;
        }
    }

    count
}

fn countValleys4(s: String) -> i32 {
    let mut count = 0;
    let mut current = 0;
    let mut prev: i32;

    for c in s.chars() {
        prev = current;
        if c == 'U' { current += 1; }
        if c == 'D' { current -= 1; }
        if current == 0 && prev == -1 { count += 1; }
    }

    count
}

fn countValleys5(s: String) -> i32 {
    let mut count = 0;
    let mut current = 0;
    let mut prev: i32;

    for c in s.chars() {
        prev = current;
        let delta = match c {
            'U' =>  1,
            'D' => -1,
            _ => 0,
        };
        current += delta;
        if current == 0 && prev == -1 { count += 1; }
    }

    count
}

#[test]
fn test1() {}
