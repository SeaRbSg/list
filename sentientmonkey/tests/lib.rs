extern crate list;
use list::eval;

#[test]
fn can_eval() {
    let value = "test".to_string();
    assert_eq!(value, eval(value.clone()));
}
