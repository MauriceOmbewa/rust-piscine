pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let division = x/y;
    let remainder = x%y;

    let my_tuple = (division, remainder);
    my_tuple
}