extern crate libfornix;

use libfornix::*;

#[test]
fn simple_input() {
    let input = SimpleInput::new(5.0f64);

    assert_eq!(input.read(0), 5.0f64);
    assert_eq!(input.len(), 1);
}

fn input_list() {
    let mut inputs = ProgramInputs::new();
    let input = SimpleInput::new(5.0f64);

    inputs = inputs.add(Box::new(input));

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs.get(0).read(0), 5.0f64);

    let mut ran = false;
    for i in inputs {
        ran = true;
        assert_eq!(i.read(0), 5.0f64);
    }
    assert!(ran);
}

