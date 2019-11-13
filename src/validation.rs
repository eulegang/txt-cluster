pub fn nonnegative(value: String) -> Result<(), String> {
    match i32::from_str_radix(&value, 10) {
        Ok(i) if i < 0 => Err(format!("{} is negative", i)),
        Err(_) => Err(format!("{} not a number", value)),
        _ => Ok(()),
    }
}

pub fn ratio(value: String) -> Result<(), String> {
    match value.parse::<f64>() {
        Ok(f) if 0.0 < f && f < 1.0 => Ok(()),
        Ok(_) => Err(format!("{} is not between 0 and 1", value)),
        Err(_) => Err(format!("{} is not a float", value)),
    }
}
