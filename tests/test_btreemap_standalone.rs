#[path = "../src/klib/vec.rs"]
pub mod vec;
pub use vec::Vec;

#[path = "../src/klib/btreemap.rs"]
pub mod btreemap;

#[path = "../src/klib/hashset.rs"]
pub mod hashset;

use btreemap::BTreeMap;
use hashset::HashSet;

#[test]
fn test_btreemap_insert_return_option() {
    let mut map = BTreeMap::new();
    // First insertion returns None
    assert_eq!(map.insert("key1", 10), None);
    assert_eq!(map.insert("key2", 20), None);
    assert_eq!(map.len(), 2);

    // Overwriting returns Previous value (Some)
    assert_eq!(map.insert("key1", 15), Some(10));
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&15));
}

#[test]
fn test_btreemap_get_and_remove() {
    let mut map = BTreeMap::new();
    map.insert(5, "five");
    map.insert(2, "two");
    map.insert(8, "eight");

    assert_eq!(map.get(&5), Some(&"five"));
    assert_eq!(map.get(&2), Some(&"two"));
    assert_eq!(map.get(&8), Some(&"eight"));
    assert_eq!(map.get(&10), None);

    assert_eq!(map.remove(&2), Some("two"));
    assert_eq!(map.get(&2), None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_btreemap_entry_api() {
    let mut map = BTreeMap::new();
    *map.entry("counter").or_insert(0) += 1;
    *map.entry("counter").or_insert(0) += 5;

    assert_eq!(map.get("counter"), Some(&6));
}

#[test]
fn test_hashset_single_pass_insert() {
    let mut set = HashSet::new();
    assert!(set.is_empty());

    // First insert returns true
    assert!(set.insert(100));
    assert!(set.insert(200));
    assert_eq!(set.len(), 2);

    // Duplicate insert returns false
    assert!(!set.insert(100));
    assert_eq!(set.len(), 2);

    assert!(set.contains(&100));
    assert!(set.contains(&200));
    assert!(!set.contains(&300));

    assert!(set.remove(&100));
    assert!(!set.contains(&100));
    assert_eq!(set.len(), 1);
}
