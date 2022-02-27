use wasm_rust;

#[test]
fn test_add() {
  assert_eq!(wasm_rust::add::add(1, 1), 2);
}
