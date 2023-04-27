use bobcalc;

#[test]
fn one_num() {
    assert_eq!(bobcalc::compute("16"), 16 as f32);
}

#[test]
#[should_panic(expected = "")]
fn operator_missing() {
    bobcalc::compute("+");
}
