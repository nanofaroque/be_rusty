fn main() {
    let mut start_time = Vec::new();
    start_time.push(1);
    start_time.push(2);
    start_time.push(3);

    let mut end_time = Vec::new();
    end_time.push(3);
    end_time.push(2);
    end_time.push(7);

    let query_time=4;
    let res= busy_student(start_time,end_time,query_time);
    println!("{}", res)

}

pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let mut counter=0;
    for i in 0..start_time.len() {
        println!("{}", start_time[i]);
        let start = start_time[i];
        let end = end_time[i];
        if query_time>=start && query_time<=end{
            counter+=1;
        }
    }
    return counter
}
