extern crate list;
use list::eval;

#[test]
fn can_eval_number() {
    let value = Ok(list::Lval::Num(42));
    assert_eq!(value, eval(value.clone()));
}
