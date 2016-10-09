extern crate libfornix;

#[test]
fn testRandomness() {
    let tests = 10000000;
    let max_equality = 0.001;

    let provider = libfornix::OsRandomNumberProvider::new();

    //let mut testResult = Vec::new();

    // TODO: make sure equality is under a specified limit to make sure randomness
}