use std::fs;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

use self::error::Result;
mod error;
mod ser;
// TODO: Implement deserialization
mod de;

#[allow(dead_code)]
pub fn from_string<T>(data: &str) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    de::from_str::<T>(data)
}

#[allow(dead_code)]
pub fn to_string<T>(obj: &T) -> Result<String>
where
    T: Serialize,
{
    ser::to_string(obj)
}

#[allow(dead_code)]
pub fn from_file<T>(file_path: &str) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let buf = fs::read_to_string(file_path)?;
    let res = de::from_str::<T>(&buf);
    match res {
        Ok(obj) => Ok(obj),
        Err(err) => {
            anyhow::bail!(err)
        }
    }
}

#[allow(dead_code)]
pub fn to_file<T>(obj: &T, file_path: &str) -> anyhow::Result<()>
where
    T: Serialize,
{
    let serial = ser::to_string(obj)?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(&serial.into_bytes())?;
    Ok(())
}

#[cfg(test)]

mod tests {
    // use super::*;
    // TODO: Add ser + de tests here
}
