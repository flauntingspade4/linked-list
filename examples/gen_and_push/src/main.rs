use linked_list::generate_head;

fn main() {
    //The function 'generate_head' is included to make it easier to make lists
    let mut head = generate_head(vec!["Hello", ", ", "World!"]);

    //You can then add more items with relative ease
    head.add_new("Hello, Rust!");

    //You can then get the length of the linked list
    println!("{}", head.len());
}
