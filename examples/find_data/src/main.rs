use linked_list::generate_head;

fn main() {
    //As before, I'll generate a new head with the function 'generate_head'
    let head = generate_head(vec![0, 7, 12, 98, 2, 5]);

    //You can find a refrence to, and the index of, a piece of given information using 'head.find();'
    //This will return an Option<(&LinkedList<T>, usize)>
	let (link, index) = head.find(4).unwrap();
	
	//This will print 'The index 4 has the value 2.'
	println!("The index {} has the value {}.", index, link.get_data());
}
