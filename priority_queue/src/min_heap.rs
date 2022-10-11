
use std::collections::BinaryHeap;
use std::cmp::Ordering;


/**
 * 
 * Output: 
 * Name: omar, Age: 30 
 * Name: faroque, Age: 31 
 * Name: Pink, Age: 32 
*/

struct Person{
    age: i32,
    name: String
}

impl Eq for Person {}


impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.age.cmp(&other.age).reverse()
    }
}


impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.age == other.age
    }
}

fn main() {

    let a =Person{age: 30,name:"omar".to_string()};
    let b =Person{age: 31,name:"faroque".to_string()};
    let c =Person{age: 32,name:"Pink".to_string()};
    
    let mut heap:BinaryHeap<Person> = BinaryHeap::new();
    
    heap.push(a);
    heap.push(b);
    heap.push(c);
    
    let p=heap.pop().unwrap();
    println!("Name: {}, Age: {} ",p.name,p.age);

    let q=heap.pop().unwrap();
    println!("Name: {}, Age: {} ",q.name,q.age);

    let r=heap.pop().unwrap();
    println!("Name: {}, Age: {} ",r.name,r.age);
}
