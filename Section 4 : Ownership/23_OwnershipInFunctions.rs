fn main(){
    let vec_1= Vec<i32>=vec![1,2,3];
    let vec_2= Vec<i32>=vec![1,2,3];
    takes_ownership(vec_1); // ownership of vec_1 is passed to function
    take_ownership(vec_2.clone())// Will create new instance and ownership of new instance is transferred
    print!("vec 1 is :{:?} , vec_1";) // this will give error
    print!("vec 2 is :{:?} , vec_2") // this won't give error

    gives_ownership();
    let vec_3=gives_ownership(); // ownership is transferred to vec_3
    let vec_4=take_n_give(vec_3.clone()); // take and give simultaneously

    let x:i32=10;
    stack_function(x); // copy is created in case of primitive data types ownership is not transferred

fn  takes_ownership(vec:Vec<i32>)
{
    // code here
}
// At the end of the function execution vec_1 will be cleared from the heap

fn gives_ownership() -> Vec<i32> 
{
    vec![4,5,6]
}

fn take_n_give(vec:Vec<i32>)
{
    vec![4,5,6];
}

fn stack_function(mut var:32)
{
    // code here
}