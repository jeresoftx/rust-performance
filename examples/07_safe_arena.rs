use rust_performance::arena::SafeArena;

fn main() {
    let mut arena = SafeArena::with_capacity(2);
    let id = arena.insert("temporal");
    println!("{:?}", arena.get(id));
    arena.reset();
    println!("{:?}", arena.get(id));
}
