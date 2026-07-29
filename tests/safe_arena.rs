use rust_performance::arena::SafeArena;

#[test]
fn retrieves_values_inserted_in_the_current_generation() {
    let mut arena = SafeArena::with_capacity(2);
    let id = arena.insert("valor");

    assert_eq!(arena.get(id), Some(&"valor"));
}

#[test]
fn reset_invalidates_previous_identifiers() {
    let mut arena = SafeArena::new();
    let old = arena.insert(7);
    arena.reset();
    let current = arena.insert(9);

    assert_eq!(arena.get(old), None);
    assert_eq!(arena.get(current), Some(&9));
}
