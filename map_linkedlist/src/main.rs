use std::collections::{ HashMap, LinkedList};


fn main() {
    /**
    1->1,1
    2->2,2
    */

    let mut m: HashMap<u32,LinkedList<u32>> = HashMap::new();
    let xs: [u32; 10] = [1, 2, 3, 4, 5,1, 2, 3, 4, 5];
    for i in xs.iter(){
        if m.contains_key(&i) {
            println!("contains key");
            let list:&mut LinkedList<u32>=m.get_mut(&i).unwrap();
            list.push_back(*i);
        }else {
            &m.insert(*i,LinkedList::new());
            let list:&mut LinkedList<u32>=m.get_mut(&i).unwrap();
            list.push_back(*i);
        }
    }

    for (key, value) in m.into_iter() {
        let mut iter = value.iter();
        print!("{},{}",key.to_string(),"->");
        for v in value.iter(){
            print!("{}",v.to_string())
        }
        println!("{}","");
    }
}

