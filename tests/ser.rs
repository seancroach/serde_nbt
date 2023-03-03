use serde_nbt::{to_value, value::Byte, Value};

#[test]
fn to_value_bool() -> serde_nbt::Result<()> {
    let true_value = to_value(&true)?;
    assert_eq!(true_value, Value::Byte(Byte::Boolean(true)));

    let false_value = to_value(&false)?;
    assert_eq!(false_value, Value::Byte(Byte::Boolean(false)));

    Ok(())
}

#[test]
fn to_value_i8() -> serde_nbt::Result<()> {
    let i8_min = to_value(&i8::MIN)?;
    assert_eq!(i8_min, Value::Byte(Byte::Integer(i8::MIN)));

    let i8_max = to_value(&i8::MAX)?;
    assert_eq!(i8_max, Value::Byte(Byte::Integer(i8::MAX)));

    Ok(())
}

#[test]
fn to_value_i16() -> serde_nbt::Result<()> {
    let i16_min = to_value(&i16::MIN)?;
    assert_eq!(i16_min, Value::Short(i16::MIN));

    let i16_max = to_value(&i16::MAX)?;
    assert_eq!(i16_max, Value::Short(i16::MAX));

    Ok(())
}

#[test]
fn to_value_i32() -> serde_nbt::Result<()> {
    let i32_min = to_value(&i32::MIN)?;
    assert_eq!(i32_min, Value::Int(i32::MIN));

    let i32_max = to_value(&i32::MAX)?;
    assert_eq!(i32_max, Value::Int(i32::MAX));

    Ok(())
}

#[test]
fn to_value_i64() -> serde_nbt::Result<()> {
    let i64_min = to_value(&i64::MIN)?;
    assert_eq!(i64_min, Value::Long(i64::MIN));

    let i64_max = to_value(&i64::MAX)?;
    assert_eq!(i64_max, Value::Long(i64::MAX));

    Ok(())
}

#[test]
fn to_value_f32() -> serde_nbt::Result<()> {
    let f32_min = to_value(&f32::MIN)?;
    assert_eq!(f32_min, Value::Float(f32::MIN));

    let f32_max = to_value(&f32::MAX)?;
    assert_eq!(f32_max, Value::Float(f32::MAX));

    Ok(())
}

#[test]
fn to_value_f64() -> serde_nbt::Result<()> {
    let f64_min = to_value(&f64::MIN)?;
    assert_eq!(f64_min, Value::Double(f64::MIN));

    let f64_max = to_value(&f64::MAX)?;
    assert_eq!(f64_max, Value::Double(f64::MAX));

    Ok(())
}

#[test]
fn to_value_string() -> serde_nbt::Result<()> {
    let empty_string = to_value("")?;
    assert_eq!(empty_string, Value::String(String::new()));

    let greeting = to_value("Hello, world!")?;
    assert_eq!(greeting, Value::String(String::from("Hello, world!")));

    Ok(())
}
