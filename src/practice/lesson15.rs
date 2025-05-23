/// pack_seq, unpack_seq
// [ 'a',      'b', 'b',  'c', 'c', 'c',   'a']
// [('a', 1), ('b', 2),  ('c',        3), ('a', 1)]

// [ 'a',      'b',     'a',      'b',      'c', 'c', 'c', 'a']
// [('a', 1), ('b', 1),('a', 1), ('b', 1), ('c', 3),      ('a', 1)]
fn pack1(xs: &Vec<char>) -> Vec<(char, u32)> {
    let mut outcome = Vec::new();

    for &c in xs {
        if outcome.is_empty() {
            outcome.push((c, 1));
        } else {
            let last = outcome.last_mut().unwrap();
            if last.0 == c {
                // increment
                last.1 += 1;
            } else {
                // just add
                outcome.push((c, 1));
            }
        }
    }

    outcome
}

fn pack(xs: &Vec<char>) -> Vec<(char, u32)> {
    let mut outcome = Vec::new();

    for &c in xs {
        match outcome.last_mut() {
            Some((ch, cnt)) if *ch == c => *cnt += 1,
            _ => outcome.push((c, 1)),
        }
    }

    outcome
}

fn unpack(xs: Vec<(char, u32)>) -> Vec<char> {
    todo!()
}

#[test]
fn test1() {
    let test_data = [
        (
            vec!['a', 'b', 'b', 'c', 'c', 'c', 'a'],
            vec![('a', 1), ('b', 2), ('c', 3), ('a', 1)],
        ),
        (
            vec!['a', 'b', 'a', 'b', 'c', 'c', 'c', 'a'],
            vec![('a', 1), ('b', 1), ('a', 1), ('b', 1), ('c', 3), ('a', 1)],
        ),
    ];
    for (input, expected) in test_data {
        let packed = pack(&input);
        println!("{:?}", input);
        println!("{:?}", packed);

        assert_eq!(packed, expected);
    }
}
