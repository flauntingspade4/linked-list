use linked_list::{generate_head, LinkedHead, LinkedList};

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
    head.add_new(5);

    assert_eq!(head.len(), 1);
}

#[test]
fn test_find() {
    let mut head = LinkedHead::new();
    head.add_new(5);
    head.add_new(5);
    head.add_new(6);

    assert_eq!(head.len(), 3);
    assert_eq!(head.find(6).unwrap().0, &LinkedList::new(6));
}

#[test]
fn test_set_data_indexed() {
    let mut head = LinkedHead::new();
    head.add_new(5);
    head.add_new(5);
    head.add_new(6);

    assert_eq!(head.len(), 3);
    assert_eq!(head.find(6).unwrap().0, &LinkedList::new(6));

    head.set_data(2, 2);

    assert_eq!(head.get_last().unwrap().get_data(), &2);
}

#[test]
fn get_last_get_first() {
    let mut head = LinkedHead::new();
    head.add_new(-752);
    head.add_new(980);
    head.add_new(6);

    assert_eq!(head.get_first().unwrap().get_data(), &-752);
    assert_eq!(head.get_last().unwrap().get_data(), &6);
}

#[test]
fn get_first_mut() {
    let mut head = LinkedHead::new();
    head.add_new(-752);
    head.add_new(980);
    head.add_new(6);

    assert_eq!(head.get_first_mut().unwrap().get_data_mut(), &mut -752);
}

#[test]
fn get_at_point() {
    let mut head = generate_head(vec![5, 17, 249, -23, 425]);
    let (link, index) = head.find(249).unwrap();

    assert_eq!(link.get_data(), &249);
    assert_eq!(index, 2);

    let new_link = head.find(127);

    assert_eq!(new_link, None);
    head.set_data(2, 250);
    let data = head.get(2);
    assert_eq!(data.get_data(), &250);
}
