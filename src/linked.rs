use std::cmp::PartialEq;

pub struct LinkedHead<'a, T>
where
	T: PartialEq,
{
	pointer: Option<&'a mut LinkedList<'a, T>>,
}

impl<'a, T> LinkedHead<'a, T>
where
	T: PartialEq,
{
	pub fn new() -> Self {
		// Creates a new empty head
		Self { pointer: None }
	}
	pub fn set_pointer(&mut self, pointer: &'a mut LinkedList<'a, T>) {
		// Sets the pointer of the head
		self.pointer = Some(pointer);
	}
	pub fn find(&self, to_find: T) -> Option<&'a LinkedList<T>> {
		// Returns either a refrence to the first link in the list containing the data, and nothing if none contain said data
		match &self.pointer {
			Some(pointer) => pointer.find(to_find),
			None => None,
		}
	}
	pub fn set_data(&mut self, index: usize, data: T) {
		// Sets the data of the list at a given index, panicing if that index is out of bounds
		match &mut self.pointer {
			Some(pointer) => pointer.set_data_indexed(0, index, data),
			None => {}
		}
	}
}

pub struct LinkedList<'a, T>
where
	T: PartialEq,
{
	data: T,
	pointer: Option<&'a mut LinkedList<'a, T>>,
}

impl<'a, T> LinkedList<'a, T>
where
	T: PartialEq,
{
	pub fn new(data: T) -> Self {
		// Makes a new link in the list, with a piece of given data
		Self {
			data,
			pointer: None,
		}
	}
	pub fn set_pointer(&mut self, pointer: &'a mut LinkedList<'a, T>) {
		// Sets the link's pointer
		self.pointer = Some(pointer);
	}
	pub fn set_data(&mut self, data: T) {
		// Sets the link's data
		self.data = data;
	}
	fn set_data_indexed(&mut self, current_index: usize, index: usize, data: T) {
		// Checks if the link's index is the requested index, if so it'll set the data to the given data. If the requested index is out of bounds, it'll panic
		if current_index == index {
			self.set_data(data);
		} else {
			match &mut self.pointer {
				Some(pointer) => {
					pointer.set_data_indexed(current_index + 1, index, data);
				}
				None => panic!("Tried to index number currently out of bounds for linked list"),
			}
		}
	}
	pub fn find(&self, to_find: T) -> Option<&LinkedList<'a, T>> {
		// Will return Some(&self) if the current data is the same as the requested data, and none if there's none in the list
		if self.data == to_find {
			return Some(&self);
		}
		match &self.pointer {
			Some(pointer) => Some(pointer.find(to_find).unwrap()),
			None => None,
		}
	}
}
