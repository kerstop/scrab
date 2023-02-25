use super::*;

/// testing that the size of the boards generated with the `new`
/// function are correct
#[test]
fn new_sizes() {
    let grid_zero = HexGrid::<String>::new(0);
    let grid_one = HexGrid::<String>::new(1);
    let grid_two = HexGrid::<String>::new(2);
    let grid_three = HexGrid::<String>::new(3);
    let grid_four = HexGrid::<String>::new(4);
    let grid_five = HexGrid::<String>::new(5);

    assert_eq!(grid_zero.len(), 0);
    assert_eq!(grid_one.len(), 1);
    assert_eq!(grid_two.len(), 7);
    assert_eq!(grid_three.len(), 19);
    assert_eq!(grid_four.len(), 37);
    assert_eq!(grid_five.len(), 61);
}

#[test]
fn cube_to_index_test() {
    vec![
        (0, 0, 0),
        (0, -1, 1),
        (1, -1, 0),
        (1, 0, -1),
        (0, 1, -1),
        (-1, 1, 0),
        (-1, 0, 1),
        (0, -2, 2),
        (1, -2, 1),
        (2, -2, 0),
        (2, -1, -1),
        (2, 0, -2),
        (1, 1, -2),
        (0, 2, -2),
        (-1, 2, -1),
        (-2, 2, 0),
        (-2, 1, 1),
        (-2, 0, 2),
        (-1, -1, 2),
        (0, -3, 3),
        (1, -3, 2),
        (2, -3, 1),
        (3, -3, 0),
        (3, -2, -1),
        (3, -1, -2),
        (3, 0, -3),
        (2, 1, -3),
        (1, 2, -3),
        (0, 3, -3),
        (-1, 3, -2),
        (-2, 3, -1),
        (-3, 3, 0),
        (-3, 2, 1),
        (-3, 1, 2),
        (-3, 0, 3),
        (-2, -1, 3),
        (-1, -2, 3),
    ]
    .iter()
    .enumerate()
    .for_each(|(i, (q, r, s))| assert_eq!(Cordinate::from_cube(*q, *r, *s).unwrap().as_index(), i));
}
