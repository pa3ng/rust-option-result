use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Debug, Default, Deserialize, Serialize)]
struct AStruct {
    #[serde(skip_serializing_if = "is_none_or_default")]
    string: Option<String>,
    #[serde(skip_serializing_if = "is_none_or_default")]
    integer: Option<i32>,
    #[serde(skip_serializing_if = "is_none_or_default")]
    vector: Option<Vec<bool>>,
}

fn is_none_or_default<T: Default + PartialEq>(obj: &Option<T>) -> bool {
    match obj {
        Some(o) => *o == T::default(),
        None => true,
    }
}

fn print_struct(a_struct: AStruct) -> Result<(), Error> {
    println!("a_struct as passed: {:?}", a_struct);

    let json = serde_json::to_string(&a_struct)?;
    println!("a_struct serialized to JSON: {}", json);

    let destruct = serde_json::from_str::<AStruct>(&json)?;
    println!("JSON deserialized back to AStruct: {:?}", destruct);

    Ok(())
}

fn main() -> Result<(), Error> {
    print_struct(AStruct {
        ..Default::default()
    })?;
    println!();

    print_struct(AStruct {
        string: None,
        integer: None,
        vector: None,
    })?;
    println!();
    print_struct(AStruct {
        string: Some(String::default()),
        integer: Some(i32::default()),
        vector: Some(Vec::default()),
    })?;
    println!();

    print_struct(AStruct {
        string: Some(String::from("")),
        integer: Some(0),
        vector: Some(vec![]),
    })?;
    println!();

    print_struct(AStruct {
        string: Some(String::from("a string")),
        integer: Some(42),
        vector: Some(vec![true, false]),
    })
}
