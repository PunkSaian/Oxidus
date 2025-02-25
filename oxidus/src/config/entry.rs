use std::collections::HashMap;

use toml::{Table, Value};


#[derive(Debug, Clone, PartialEq)]
pub enum EntryValue {
    I32(i32),
    F32(f32),
    String(String),
    Bool(bool),
    Color(i32),
}

#[derive(Debug, Clone)]
pub enum Entry {
    Value(EntryValue, EntryValue, Option<EntryValue>),
    Group(HashMap<String, Entry>),
}

impl Entry {
    pub fn merge(self, loaded: &Value) -> Entry {
        match (self, loaded) {
            (Entry::Value(EntryValue::I32(_), def, _), Value::Integer(v)) => {
                Entry::Value(EntryValue::I32(*v as i32), def.clone(), None)
            }
            (Entry::Value(EntryValue::F32(_), def, _), Value::Float(v)) => {
                Entry::Value(EntryValue::F32(*v as f32), def.clone(), None)
            }
            (Entry::Value(EntryValue::String(_), def, _), Value::String(v)) => {
                Entry::Value(EntryValue::String(v.clone()), def.clone(), None)
            }
            (Entry::Value(EntryValue::Bool(_), def, _), Value::Boolean(v)) => {
                Entry::Value(EntryValue::Bool(*v), def.clone(), None)
            }
            (Entry::Value(EntryValue::Color(_), def, _), Value::String(v)) => {
                let hex = v.trim_start_matches('#');
                Entry::Value(
                    EntryValue::Color(i32::from_str_radix(hex, 16).unwrap()),
                    def.clone(),
                    None,
                )
            }
            (Entry::Group(cur_map), Value::Table(loaded_table)) => {
                let mut merged_map = HashMap::new();
                for (key, value) in cur_map {
                    if let Some(loaded_val) = loaded_table.get(&key) {
                        let merged = value.merge(loaded_val);
                        merged_map.insert(key.clone(), merged);
                    } else {
                        merged_map.insert(key.clone(), value.clone());
                    }
                }
                Entry::Group(merged_map)
            }
            (entry, _) => entry,
        }
    }
    pub fn to_toml(&self) -> Value {
        match self {
            Entry::Value(value, ..) => match value {
                EntryValue::I32(v) => Value::Integer(i64::from(*v)),
                EntryValue::F32(v) => Value::Float(f64::from(*v)),
                EntryValue::String(v) => Value::String(v.clone()),
                EntryValue::Bool(v) => Value::Boolean(*v),
                EntryValue::Color(v) => Value::String(format!("#{v:06X}")),
            },
            Entry::Group(map) => {
                let mut table = Table::new();
                for (key, value) in map {
                    table.insert(key.clone(), value.to_toml());
                }
                Value::Table(table)
            }
        }
    }
}
