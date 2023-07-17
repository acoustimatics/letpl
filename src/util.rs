pub fn ok_box<TVal, TErr>(value: TVal) -> Result<Box<TVal>, TErr> {
    Ok(Box::new(value))
}
