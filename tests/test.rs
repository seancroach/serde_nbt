#[cfg(feature = "snbt")]
use serde_nbt::{to_string, to_string_pretty};

use serde::Serialize;

#[track_caller]
fn test_to_string<T>(value: &T, normalize: bool, expected: &'static str)
where
    T: ?Sized + Serialize,
{
    #[cfg(feature = "snbt")]
    {
        let actual = to_string(value, normalize).unwrap();
        assert_eq!(actual, expected)
    }
    #[cfg(not(feature = "snbt"))]
    {
        let _ = (value, expected);
    }
}

#[track_caller]
fn test_to_string_pretty<T>(value: &T, normalize: bool, expected: &'static str)
where
    T: ?Sized + Serialize,
{
    #[cfg(feature = "snbt")]
    {
        let actual = to_string_pretty(value, normalize).unwrap();
        assert_eq!(actual, expected)
    }
    #[cfg(not(feature = "snbt"))]
    {
        let _ = (value, expected);
    }
}

#[test]
fn test_serialize_bool() {
    test_to_string(&false, false, "false");
    test_to_string_pretty(&false, false, "false");

    test_to_string(&true, false, "true");
    test_to_string_pretty(&true, false, "true");
}

#[test]
fn test_serialize_i8() {
    test_to_string(&-128i8, false, "-128b");
    test_to_string_pretty(&-128i8, false, "-128b");

    test_to_string(&127i8, false, "127b");
    test_to_string_pretty(&127i8, false, "127b");
}

#[test]
fn test_serialize_i16() {
    test_to_string(&-32_768i16, false, "-32768s");
    test_to_string_pretty(&-32_768i16, false, "-32768s");

    test_to_string(&32_767i16, false, "32767s");
    test_to_string_pretty(&32_767i16, false, "32767s");
}
