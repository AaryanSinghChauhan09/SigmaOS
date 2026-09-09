#[path = "../src/klib/hash.rs"]
pub mod hash;

#[path = "../src/klib/hashmap.rs"]
pub mod hashmap;

use hashmap::HashMap;

#[test]
fn test_hashmap_basic_operations() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    map.insert("alpha", 1);
    map.insert("beta", 2);
    map.insert("gamma", 3);

    assert_eq!(map.len(), 3);
    assert_eq!(map.get("alpha"), Some(&1));
    assert_eq!(map.get("beta"), Some(&2));
    assert_eq!(map.get("gamma"), Some(&3));
    assert_eq!(map.get("delta"), None);

    assert!(map.contains_key("beta"));
    assert!(!map.contains_key("delta"));
}

#[test]
fn test_hashmap_overwrite_and_remove() {
    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("key1", "val1".to_string());
    map.insert("key1", "val1_updated".to_string());

    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key1"), Some(&"val1_updated".to_string()));

    assert_eq!(map.remove("key1"), Some("val1_updated".to_string()));
    assert_eq!(map.len(), 0);
    assert_eq!(map.get("key1"), None);
}

#[test]
fn test_hashmap_entry_api() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    *map.entry("counter").or_insert(0) += 1;
    *map.entry("counter").or_insert(0) += 5;

    assert_eq!(map.get("counter"), Some(&6));
}

#[test]
fn test_hashmap_capacity_and_growth() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(4);
    for i in 0..100 {
        map.insert(i, i * 10);
    }

    assert_eq!(map.len(), 100);
    for i in 0..100 {
        assert_eq!(map.get(&i), Some(&(i * 10)));
    }
}

#[test]
fn test_hashmap_iteration() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(10, 100);
    map.insert(20, 200);

    let sum: i32 = map.iter().map(|(_, v)| v).sum();
    assert_eq!(sum, 300);
}
