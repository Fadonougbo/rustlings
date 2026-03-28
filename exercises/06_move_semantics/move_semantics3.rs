// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec=vec;
    vec.push(88);

    vec
}

fn main() {
    
    // let mut vec:Vec<i32>= vec![];

    // vec.push(30);

    // println!("{}",vec[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
