use std::convert::TryInto;

fn main() {
   let res=  is_prefix_of_word(String::from("i love eating burger"),
                      String::from("burg"));
    println!("{}",res);
}

pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    let str_vec: Vec<&str> = sentence.split(' ').collect();
    let count=-1;
    for i  in 0..str_vec.len() {
        let w = str_vec[i];

        if search_word.len()<=w.len()  {
            let slice = &w[..search_word.len()];
            if slice==search_word {
                //println!("{}",count+1+);
                return (i+1) as i32
            }
        }
    }
    return count;

}