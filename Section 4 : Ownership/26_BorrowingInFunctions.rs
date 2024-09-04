
fn main()
{
    let vec_1: Vec<i32> = vec![1,2,3];
    let ref1:&Vec<i32>= &vec_1;
    // let ref2:& Vec<i32> = &mut vec_1 ; `this can also be not done because of two rules`
    borrow_reference(ref1);
    take_ownership(vec_1);
    // take_ownership(vec_1.clone()); `this will not be good because it will allocate memory on heap.`
    
    /* 
      take_n_give(vec_1);
      `if now will call line no 17 it will give an error, therefore it will be good to do shadowing`
      i.e, let vec_1: Vec<i32>=  take_n_give(vec_1);
      `Now linke 17 can be called, but this is not efficient.`
    */
    println!("vec 1 is : {:?} ", vec_1); // this will not work
    /*
      -A mutable reference can be passed in instead of ownership in this case
      -Mutable reference can only be maded for mutable variables.
        let mut vec2: Vec<i32> = vec![1,2,3];
        let ref2: &mut Vec<i32>=&mut vec_2;
        mutably_borrows_vec(ref2);
    */
}

fn  takes_ownership(vec:Vec<i32>)
{
    // code here, ownership not required by function but given
}


fn  borrow_reference(vec: & Vec<i32>) // reference is given
{
    // code here, ownership not required therefore not given
}

fn take_n_give(mut vec:Vec<i32>)->Vec<i32>
{
    vec.push(10);
    vec
}

fn muttably_borrow(vec:&mut Vec<i32>) // No need to return
{
    vec.push(10);
    vec
}

// Case of dangling references
/* 
    fn gives_ownership()-> &Vec<i32>
    {
        let vec1=!vec![4,5,6];
        &vec1
    }
*/

/*
    -when you create a value within a function and intend to return it, you must transfer ownership of that value.
    -Returning a reference is not advisable because the old value will be automatically cleaned up at the function's end, rendering the reference invalid.
*/