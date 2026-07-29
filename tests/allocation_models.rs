use rust_performance::allocation::{build_message_fresh, build_message_reused};

#[test]
fn fresh_and_reused_construction_produce_the_same_message() {
    let mut buffer = Vec::with_capacity(32);

    assert_eq!(
        build_message_fresh("hola"),
        build_message_reused(&mut buffer, "hola")
    );
}

#[test]
fn reused_buffer_does_not_leak_previous_message_bytes() {
    let mut buffer = Vec::with_capacity(32);
    let _ = build_message_reused(&mut buffer, "mensaje muy largo");

    assert_eq!(build_message_reused(&mut buffer, "corto"), b"msg:corto");
}
