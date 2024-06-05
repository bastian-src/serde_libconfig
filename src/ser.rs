// Implement like: https://serde.rs/impl-serializer.html

use core::option::Option;
use serde::{self, ser, Serialize};

use super::error::{Error, Result};

pub struct Serializer {
    output: String,
    indentation_level: u64,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        indentation_level: 0,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i32(i32::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        self.output += "L";
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u32(u32::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u32(u32::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        self.output += "UL";
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += "\"";
        self.output += v;
        self.output += "\"";
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output += "null";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // self.output += "{";
        //variant.serialize(&mut *self)?;
        self.output += variant;
        self.output += " = ";
        value.serialize(&mut *self)?;
        self.output += ";";
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output += "[";
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        // self.output += "{";
        self.output += variant;
        self.output += " = [";
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        if !self.output.is_empty() {
            if self.output.ends_with("= ") {
                self.output += "{\n";
                self.indentation_level += 1;
                self.serialize_map(Some(len))?;
                Ok(self)
            } else {
                self.output += name;
                self.output += " = {\n";
                self.serialize_map(Some(len))
            }
        } else {
            self.serialize_map(Some(len))
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        // self.output += "{";
        //variant.serialize(&mut *self)?;
        self.output += variant;
        self.output += " = {\n";
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "];";
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += " = ";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += ";";
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.output.ends_with(';') {
            self.output += "\n";
        }
        for _ in 0..self.indentation_level {
            self.output += "    ";
        }
        self.output += key;
        self.output += " = ";
        value.serialize(&mut **self)?;
        self.output += ";";
        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
            self.output += "\n}";
        }
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.output.ends_with(';') {
            self.output += "\n";
        }
        for _ in 0..self.indentation_level {
            self.output += "    ";
        }
        self.output += key;
        self.output += " = ";
        value.serialize(&mut **self)?;
        self.output += ";";
        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
        }
        self.output += "\n};";
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
///// TESTS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]

mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq: Vec<&'static str>,
    }

    #[test]
    fn test_struct() {
        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };
        let expected = "int = 1;\nseq = [\"a\",\"b\"];";
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    ////////////////////////////////////////////////////////////////////////////////

    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    #[test]
    fn test_enum() {
        let u = E::Unit;
        let expected = "\"Unit\"";
        assert_eq!(to_string(&u).unwrap(), expected);
    }

    #[test]
    fn test_enum_newtype() {
        let n = E::Newtype(1);
        let expected = "Newtype = 1;";
        assert_eq!(to_string(&n).unwrap(), expected);
    }

    #[test]
    fn test_enum_tuple() {
        let t = E::Tuple(1, 2);
        let expected = "Tuple = [1,2];";
        assert_eq!(to_string(&t).unwrap(), expected);
    }

    #[test]
    fn test_enum_struct() {
        let s = E::Struct { a: 1 };
        let expected = "Struct = {\na = 1;\n};";
        assert_eq!(to_string(&s).unwrap(), expected);
    }

    ////////////////////////////////////////////////////////////////////////////////

    #[derive(Debug, Serialize)]
    struct TestBool {
        my_bool: bool,
    }

    #[derive(Debug, Serialize)]
    struct TestU16 {
        my_u16: u16,
    }

    #[derive(Debug, Serialize)]
    struct TestU64 {
        my_u64: u64,
    }

    #[derive(Debug, Serialize)]
    struct TestI16 {
        my_i16: i16,
    }

    #[derive(Debug, Serialize)]
    struct TestI64 {
        my_i64: i64,
    }

    #[derive(Debug, Serialize)]
    struct TestStr {
        my_str: String,
    }

    #[derive(Debug, Serialize)]
    struct TestU16Multi {
        my_u16: u16,
        your_u16: u16,
    }

    #[derive(Debug, Serialize)]
    struct TestStruct {
        my_u16: u16,
    }

    #[derive(Debug, Serialize)]
    struct TestNestedStruct {
        nested_struct: TestStruct,
    }

    #[derive(Debug, Serialize)]
    struct TestStructMultiVar {
        my_u16: u16,
        your_str: String,
        they_u64: u64,
    }

    #[derive(Debug, Serialize)]
    struct TestStructNestedMultiVar {
        my_u16: u16,
        nested_struct: TestStructMultiVar,
        they_u64: u64,
    }

    #[test]
    fn test_ser_struct_bool() {
        let dummy_struct = TestBool { my_bool: true };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "my_bool = true;");
    }

    #[test]
    fn test_ser_struct_i16() {
        let dummy_struct = TestI16 { my_i16: 16 };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "my_i16 = 16;");
    }

    #[test]
    fn test_ser_struct_i64() {
        let dummy_struct = TestI64 { my_i64: 16 };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "my_i64 = 16L;");
    }

    #[test]
    fn test_ser_struct_u16() {
        let dummy_struct = TestU16 { my_u16: 16 };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "my_u16 = 16;");
    }

    #[test]
    fn test_ser_struct_u64() {
        let dummy_struct = TestU64 { my_u64: 16 };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "my_u64 = 16UL;");
    }

    #[test]
    fn test_ser_struct_str() {
        let dummy_struct = TestStr {
            my_str: "ajo".to_string(),
        };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "my_str = \"ajo\";");
    }

    #[test]
    fn test_ser_struct_two_vars() {
        let dummy_struct = TestU16Multi {
            my_u16: 16,
            your_u16: 32,
        };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "my_u16 = 16;\nyour_u16 = 32;");
    }

    #[test]
    fn test_ser_nested_struct() {
        let dummy_struct = TestNestedStruct {
            nested_struct: TestStruct { my_u16: 16 },
        };
        let ser_str = to_string(&dummy_struct).unwrap();
        assert_eq!(ser_str, "nested_struct = {\n    my_u16 = 16;\n};");
    }

    #[test]
    fn test_ser_nested_struct_multi_var() {
        let dummy_struct = TestStructNestedMultiVar {
            my_u16: 321,
            nested_struct: TestStructMultiVar {
                my_u16: 16,
                your_str: "my nice schtring".to_string(),
                they_u64: 0xAAAAAAAA,
            },
            they_u64: 0xffffffff,
        };
        let ser_str = to_string(&dummy_struct).unwrap();
        let exp_str = r#"my_u16 = 321;
nested_struct = {
    my_u16 = 16;
    your_str = "my nice schtring";
    they_u64 = 2863311530UL;
};
they_u64 = 4294967295UL;"#;
        assert_eq!(ser_str, exp_str);
    }
}
