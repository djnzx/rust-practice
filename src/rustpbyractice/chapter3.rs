/// https://practice.course.rs/variables.html

#[test]
fn test30() {
    let x = 5;
    let y = 2 + 3;

    assert_eq!(x, y);
}

#[test]
fn test31() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test399() {
    let x = 5;
    let y = 10;

    let real = x + y;
    let expected = 15;

    assert_eq!(real, expected);
}