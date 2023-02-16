use serde_nbt::{to_value, Result};

use serde::Serialize;

#[test]
fn test_struct() -> Result<()> {
    #[derive(Serialize)]
    struct Test {
        int: i32,
        seq: Vec<u32>,
    }

    let test = Test {
        int: 1,
        seq: vec![1, 2, 3],
    };

    let error = to_value(&test, true).unwrap_err();
    panic!("{error}");
}
