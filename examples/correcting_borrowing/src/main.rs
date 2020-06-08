use linked_list::LinkedHead;
#[allow(dead_code)]

fn main_revised() {
    //This time I'll create, and add to, the head manually
    let mut head = LinkedHead::new();
    head.add_new(0);
    head.add_new(12);
    head.add_new(42625);
    head.add_new(123);

    //From here, I'll find the information of the item at index 2
    let information = head.get(2);

    println!("The information at index 2 is {}", information.get_data());

    //You cannot change information with any borrows anywere else.

    //`head.get_mut(0).set_data(25);` won't compile, but this will:
    let information = head.get(0);
    println!("The information at index 0 is {}", information.get_data());
}

//This can raise issues, but a workaround is the following:

fn main() {
    //This time I'll create, and add to, the head manually
    let mut head = LinkedHead::new();
    head.add_new(0);
    head.add_new(12);
    head.add_new(42625);
	head.add_new(123);
	//With these added braces it will compile, as 'information', and by extension the immutable borrow with it, won't be alive for when you change the data
    {
        //From here, I'll find the information of the item at index 2
        let information = head.get(2);

        println!("The information at index 2 is {}", information.get_data());
    }
    head.get_mut(0).set_data(25);
    println!("The information at index 0 is {}", head.get(0).get_data());
}
