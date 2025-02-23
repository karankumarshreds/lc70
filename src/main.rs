mod two_pointers;

fn main() {
    let mut v = vec![0, 1, 0, 3, 12];
    two_pointers::move_zeros::run(&mut v);
    assert_eq!(v, vec![1, 3, 12, 0, 0], "move_zeros.rs");
    let mut v = vec![2,0,1];
    two_pointers::move_zeros::run(&mut v);
    assert_eq!(v, vec![2,1,0], "move_zeros.rs");
    let mut v = vec![2,1];
    two_pointers::move_zeros::run(&mut v);
    assert_eq!(v, vec![2,1], "move_zeros.rs");
    let mut v = vec![1,2,3,1];
    two_pointers::move_zeros::run(&mut v);
    assert_eq!(v, vec![1,2,3,1], "move_zeros.rs");
}
