use linked_list::{LinkedHead, LinkedList};

#[test]
fn test_empty_head() {
    let head: LinkedHead<i32> = LinkedHead::new();

    assert_eq!(head.len(), 0);
}

#[test]
fn test_adding_link() {
    let mut head = LinkedHead::new();
    head.set_pointer(5);

    assert_eq!(head.len(), 1);
}

#[test]
fn test_adding_link_better() {
    let mut head = LinkedHead::new();
    head.add_new_pointer(5);

    assert_eq!(head.len(), 1);
}

#[test]
fn test_find() {
    let mut head = LinkedHead::new();
    head.add_new_pointer(5);
    head.add_new_pointer(5);
    head.add_new_pointer(6);

    assert_eq!(head.len(), 3);
    assert_eq!(head.find(6).unwrap(), &LinkedList::new(6));
}

#[test]
fn test_set_data_indexed() {
    let mut head = LinkedHead::new();
    head.add_new_pointer(5);
    head.add_new_pointer(5);
    head.add_new_pointer(6);

    assert_eq!(head.len(), 3);
    assert_eq!(head.find(6).unwrap(), &LinkedList::new(6));

    head.set_data(2, 2);

    assert_eq!(head.get_last().unwrap().get_data(), &2);
}

#[test]
fn get_last_get_first() {
    let mut head = LinkedHead::new();
    head.add_new_pointer(-752);
    head.add_new_pointer(980);
    head.add_new_pointer(6);

    assert_eq!(head.get_first().unwrap().get_data(), &-752);
    assert_eq!(head.get_last().unwrap().get_data(), &6);
}

#[test]
fn get_first_mut() {
    let mut head = LinkedHead::new();
    head.add_new_pointer(-752);
    head.add_new_pointer(980);
    head.add_new_pointer(6);

    assert_eq!(head.get_first_mut().unwrap().get_data_mut(), &mut -752);
}
