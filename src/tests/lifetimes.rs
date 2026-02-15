use crate::Store;

#[test]
fn test_get_ref_lifetime() {
    let mut store = Store::<String, String>::new();
    store.set("k".to_string(), "v".to_string()).unwrap();
    {
        let r = store.get_ref(&"k".to_string()).unwrap();
        assert_eq!(r, "v");
        //r goes out of scope here
    }
    store.set("k".to_string(), "w".to_string()).unwrap();
    let r2 = store.get_ref(&"k".to_string()).unwrap();
    assert_eq!(r2, "w");
}

#[test]
fn test_iter_lifetime() {
    let mut store = Store::<String, i32>::new();
    store.set("a".to_string(), 1).unwrap();
    store.set("b".to_string(), 2).unwrap();
    for (k, v) in store.iter() {

        if k == "a" {
            assert_eq!(*v, 1);
        } else if k == "b" {
            assert_eq!(*v, 2);
        } else {
            panic!("unexpected key: {}", k);
        }
    }
}