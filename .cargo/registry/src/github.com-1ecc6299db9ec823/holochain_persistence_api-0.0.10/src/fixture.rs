use crate::cas::content::Content;
use cas::content::{Address, AddressableContent};
use eav::eavi::{EntityAttributeValueIndex, ExampleAttribute, ExampleEntry};
use hash::HashString;

/// dummy hash based on the key of test_entry_a()
pub fn test_hash_a() -> HashString {
    test_entry_a().address()
}

pub fn test_entry_a() -> ExampleEntry {
    ExampleEntry::new(String::from("a"))
}

pub fn test_entry_b() -> ExampleEntry {
    ExampleEntry::new(String::from("b"))
}

pub fn test_eav_entity() -> ExampleEntry {
    test_entry_a()
}

pub fn test_eav_attribute() -> ExampleAttribute {
    ExampleAttribute::WithPayload("foo-attribute".into())
}

pub fn test_eav_value() -> ExampleEntry {
    test_entry_b()
}

pub fn test_eav() -> EntityAttributeValueIndex<ExampleAttribute> {
    EntityAttributeValueIndex::new_with_index(
        &test_eav_entity().address(),
        &test_eav_attribute(),
        &test_eav_value().address(),
        0,
    )
    .expect("Could not create eav")
}

pub fn test_eav_content() -> Content {
    test_eav().content()
}

pub fn test_eav_address() -> Address {
    test_eav().address()
}
