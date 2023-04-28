use crate::*;

#[test]
fn error() {
    let err = eval("•");
    let error_contents = "Error: System dot with no name";
    match err {
        Err(Error::CBQN(stderr)) => {
            let err = stderr.split('\n').next().unwrap();
            assert_eq!(err, error_contents);
        }
        _ => panic!("Expected an error"),
    }
}
