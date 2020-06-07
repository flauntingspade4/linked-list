use std::{cmp::PartialEq, fmt::Debug};

pub fn generate_head<T>(items: Vec<T>) -> LinkedHead<T>
where
    T: PartialEq + Debug,
{
    let mut head = LinkedHead::new();
    for item in items {
        head.add_new_pointer(item);
    }
    head
}

///The head of a linked list
#[cfg_attr(debug_assertions, derive(Debug, PartialEq))]
pub struct LinkedHead<T>
where
    T: PartialEq + Debug,
{
    pointer: Option<Box<LinkedList<T>>>,
    len: usize,
}

impl<T> LinkedHead<T>
where
    T: PartialEq + Debug,
{
    ///Creates a new empty head
    pub fn new() -> Self {
        Self {
            pointer: None,
            len: 0,
        }
    }
    ///Sets the pointer of the head
    pub fn set_pointer(&mut self, data: T) {
        let new_node = Box::new(LinkedList::new(data));

        self.pointer = Some(new_node);
        self.len += 1;
    }
    ///Returns either a refrence to the first link in the list containing the data, and nothing if none contain said data-also returns the index the data's found at
    pub fn find(&self, to_find: T) -> Option<(&LinkedList<T>, usize)> {
        match &self.pointer {
            Some(pointer) => pointer.find(to_find, 0),
            None => None,
        }
    }
    ///Sets the data of the list at a given index, panicking if that index is out of bounds
    pub fn set_data(&mut self, index: usize, data: T) {
        match &mut self.pointer {
            Some(pointer) => pointer.set_data_indexed(0, index, data),
            None => {}
        }
    }
    ///Adds a new pointer onto the end, with the specified data 'new_data'
    pub fn add_new_pointer(&mut self, new_data: T) {
        match &mut self.pointer {
            Some(pointer) => {
                pointer.try_set(new_data);
                self.len += 1;
            }
            None => self.set_pointer(new_data),
        }
    }
    ///Returns a refrence to the link at the given index
    pub fn get(&self, index: usize) -> &LinkedList<T> {
        match &self.pointer {
            Some(pointer) => pointer.get(0, index),
            None => panic!("Tried to index item out of list"),
        }
    }
    ///Returns a mutable refrence to the link at the given index
    pub fn get_mut(&mut self, index: usize) -> &mut LinkedList<T> {
        match &mut self.pointer {
            Some(pointer) => pointer.get_mut(0, index),
            None => panic!("Tried to index item out of list"),
        }
    }
    ///Returns a refrence to the last link in the list
    pub fn get_last(&self) -> Option<&LinkedList<T>> {
        match &self.pointer {
            Some(pointer) => Some(pointer.get_last()),
            None => None,
        }
    }
    ///Returns a refrence to the first link in the list
    pub fn get_first(&self) -> Option<&LinkedList<T>> {
        match &self.pointer {
            Some(pointer) => Some(pointer),
            None => None,
        }
    }
    ///Returns a mutable refrence to the first link in the list
    pub fn get_first_mut(&mut self) -> Option<&mut LinkedList<T>> {
        match &mut self.pointer {
            Some(pointer) => Some(pointer),
            None => None,
        }
    }
    ///Returns the length of the list
    pub fn len(&self) -> usize {
        self.len
    }
}

///A link in the linked list-usually won't have to be used, the [add_new_pointer](struct.LinkedHead.html#method.add_new_pointer) method is usually more appropriate to create a new link
#[cfg_attr(debug_assertions, derive(Debug, PartialEq))]
pub struct LinkedList<T>
where
    T: PartialEq + Debug,
{
    data: T,
    pointer: Option<Box<Self>>,
}

impl<T> LinkedList<T>
where
    T: PartialEq + Debug,
{
    ///Makes a new link in the list, with a piece of given data
    pub fn new(data: T) -> Self {
        Self {
            data,
            pointer: None,
        }
    }
    ///Sets the link's pointer
    pub fn set_pointer(&mut self, pointer: Self) {
        self.pointer = Some(Box::new(pointer));
    }
    ///Sets the link's data
    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }
    ///Checks if the link's index is the requested index, if so it'll set the data to the given data. If the requested index is out of bounds, it'll panic
    fn set_data_indexed(&mut self, current_index: usize, index: usize, data: T) {
        if current_index == index {
            self.set_data(data);
        } else {
            match &mut self.pointer {
                Some(pointer) => pointer.set_data_indexed(current_index + 1, index, data),
                None => panic!("Tried to index number currently out of bounds for linked list"),
            }
        }
    }
    fn find(&self, to_find: T, index: usize) -> Option<(&Self, usize)> {
        //Will return Some(&self) if the current data is the same as the requested data, and none if there's none in the list-also returns index
        if self.data == to_find {
            return Some((self, index));
        }
        match &self.pointer {
            Some(pointer) => pointer.find(to_find, index + 1),
            None => None,
        }
    }
    ///Gets a refrence to the data contained in the link
    pub fn get_data(&self) -> &T {
        &self.data
    }
    ///Gets a mutable refrence to the data contained in the link
    pub fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }
    fn try_set(&mut self, new_data: T) {
        //Checks if the current pointer is None, and if so will change the current pointer to a new
        //item, based off the new data.
        match &mut self.pointer {
            Some(pointer) => pointer.try_set(new_data),
            None => self.set_pointer(LinkedList::new(new_data)),
        }
    }
    fn get(&self, current_index: usize, index: usize) -> &Self {
		//Returns a refrence to the link at the given index
        if current_index == index {
            self
        } else {
            match &self.pointer {
                Some(pointer) => pointer.get(current_index + 1, index),
                None => panic!("Tried to index item out of list"),
            }
        }
    }
    fn get_mut(&mut self, current_index: usize, index: usize) -> &mut Self {
        //Returns a mutable refrence to the link at the given index
        if current_index == index {
            self
        } else {
            match &mut self.pointer {
                Some(pointer) => pointer.get_mut(current_index + 1, index),
                None => panic!("Tried to index item out of list"),
            }
        }
    }
    fn get_last(&self) -> &Self {
        //Returns a refrence to the last item in the list
        match &self.pointer {
            Some(pointer) => pointer.get_last(),
            None => &self,
        }
    }
}
