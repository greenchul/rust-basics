fn add_to_array(some_vec: &mut Vec<i32>)->&mut Vec<i32>{
   some_vec.push(5);
   some_vec
   
}

fn add_to_array_generic<T>(some_vec: &mut Vec<T>, item: T)->&mut Vec<T>{
    some_vec.push(item);
    some_vec
    
 }

fn get_nth_element(some_vec: & Vec<i32>, n: usize)->i32{
    some_vec[n]
}

fn get_nth_element2(some_vec: & Vec<i32>, n: usize)->i32{
    let result: Option<&i32> =some_vec.get(n);
    match result {
        Some(result) => *result,
        None => panic!("Index doesnt exist"),
    }
}

pub fn arrays(){
    let mut my_vec: Vec<i32> = vec![1, 2, 3];
    let mut my_vec2:Vec<&str> = vec!["hello", "hi"];
    println!("{:?}", add_to_array(&mut my_vec));
    println!("{:?}", add_to_array_generic(&mut my_vec2, "hii"));
    println!("{}", get_nth_element(&my_vec, 3));
    println!("{}", get_nth_element2(&my_vec, 2));
}