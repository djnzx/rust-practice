// Ellen is a new Assembly Line Manager in a shoe factory.
// So far, everything has been going very smoothly for her and N shoes of the same model and size have been
// produced. Exactly half of them are left shoes and the other half are right shoes. The freshly sewn shoes
// are standing in a line, in no particular order (i.e. with no regard to being left or right shoes).
// They now need to be matched into pairs and packed into boxes. Ellen would like to assign this task to her
// subordinate workers. Each worker should get a distinct interval of adjacent shoes, such that the number
// of left shoes is equal to the number of right shoes. Each shoe must be assigned to exactly one worker.
// What is the maximum number of workers that Ellen can assign to this task?
// 
// Write a function:
// 
// fn find_max_par(s: String) -> usize
// 
// that, given a string `s` of letters "L" and "R",
// denoting the types of shoes in line (left or right), returns the maximum number of intervals such
// that each interval contains an equal number of left and right shoes.
// 
// For example,
// given `s` = "RL RRLL RL RRLL", the function should return 4, because `s` can be split into
//                ^    ^  ^    ^
// intervals: "RL", "RRLL", "RL" and "RRLL". Note that the intervals do not have to be of the same size.
// 
// Given `s` = "RL LLRR RL LR", the function should return 4, because `s` can be split into
//                ^    ^  ^
// intervals: "RL", "LLRR", "RL" and "LR".
// 
// Given `s` = "L LR LR LR LR LR LR R", the function should return 1.
//               ^  ^  ^  ^  ^  ^  ^            
// Write an efficient algorithm for the following assumptions:
// 
// N is an integer within the range [2..100,000];
// 
// string `s` consists only of the characters "R" and/or "L";
// the number of letters "L" and "R" in string S is the same.

fn shoes_group0(s: &str, debug: bool) -> usize {
    let mut count = 0;
    let mut groups = 0;
    for c in s.chars() {
        match c {
            'L' => count += 1,
            'R' => count -= 1,
            _ => continue
        }
        if count == 0 { groups += 1; }
        if debug { print!("{count} "); }
    }
    groups
}

fn shoes_group(s: &str) -> usize {
    shoes_group0(s, false)
}

/// shoes pair
#[test]
fn test_shoes_group() {
    let test_data = [
        ("RL", 1),  
        ("LR", 1),  
        ("LLRR", 1),  
        ("LR LR", 2),  
        ("RL RL", 2),  
        ("RL RRLL RL RRLL", 4),  
        ("RL LLRR RL LR", 4),  
        ("L LR LR LR LR LR LR R", 1),  
    ];

    for (input, expected) in test_data {
        print!("testing: {input:25}");
        let real = shoes_group0(input, true);
        println!();
        assert_eq!(real, expected);
    }
    
}
