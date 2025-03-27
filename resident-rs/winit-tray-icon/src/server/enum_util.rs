// use strum::IntoEnumIterator;
// use strum_macros::EnumIter;
// use strum::EnumString;
use serde_json::{ json, Value };

const TYPE : &str = "type";

/*
  #[serde(tag = "enum")]
　でも区別できるが, 要素を列挙したものを取得したいので
*/
// requirement : use strum::IntoEnumIterator
#[macro_export]
macro_rules! define_enum { ($enum_name:ident) => {
  impl serde::Serialize for $enum_name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer, {

      use serde::ser::SerializeStruct;
      let mut state = serializer.serialize_struct(stringify!($enum_name), 3)?;

      let vec: Vec<String> = $enum_name::iter().map(|e| format!("{:?}", e)).collect();
      state.serialize_field("type", "enum")?;
      state.serialize_field("members", &vec)?;
      state.serialize_field("kind", &format!("{:?}", self))?;
      state.end()
    }
  }
}}

pub trait ToTera {
  fn to_tera(&self) -> serde_json::Value;
}

impl<T> ToTera for T where T: serde::Serialize {
  fn to_tera(&self) -> serde_json::Value { 
    let json = serde_json::to_value(&self).unwrap();
    recursive_map(&json)
  }
}

fn recursive_map( src : &serde_json::Value ) -> serde_json::Value {
  match src {
    Value::String(n) => json!({ TYPE : "string", "value" : n }),
    Value::Number(n) => json!({ TYPE : "number", "value" : n }),
    Value::Bool(n) => json!({ TYPE : "bool", "value" : n }),
    Value::Object(map) => {
      if map.contains_key("kind") {
        src.clone()
      } else {
        let mut dst = json!({});
        for (key, value) in map {
          dst[key] = recursive_map(&value);
        }
        dst
      }
    },
    Value::Array(n) => n.iter().map(|x| recursive_map(&x) ).collect(),
    Value::Null => json!({ TYPE : "null" }),
  }
}


mod tests {
  #[test]
  fn it_works() {
    use strum::IntoEnumIterator;

    #[derive(Debug, Clone, Copy, strum::EnumIter, strum::EnumString, serde::Deserialize)]
    #[strum(serialize_all = "UPPERCASE")]
    #[serde(tag = "kind", content = "data")]
    enum MyEnum {
      A,
      B,
      C,
    }
    define_enum!{ MyEnum }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct MyStruct {
      a : MyEnum
    }
    let dst = serde_json::to_string(&MyStruct{
      a : MyEnum::A
    }).unwrap();
    println!("{dst:#?}");

    println!("{:?}", serde_json::from_str::<MyStruct>(&dst));

    let v = serde_json::json!({
      "a" : 1,
      "b" : "abc",
      "c" : ["a", "b", "c"],
      "d" : {
        "type" : "enum",
        "members" : ["a", "b", "c"],
        "kind" : "b"
      }
    });

    let dst = super::recursive_map(&v);
    println!("{dst:#?}");

    let a = indoc::indoc! {r#"{
      "a" : "A"
    }"#};

    println!("{:?}", serde_json::from_str::<MyStruct>(&a));
  }
}