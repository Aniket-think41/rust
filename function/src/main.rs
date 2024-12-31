fn main() {
    println!("Hello, world!");
    first_fn();
    second_fn(5);
    third_fn(5, 'h');
    ex(7);
   let xy= return_value();
   println!("The value of xy is: {xy}");
}
fn first_fn(){
    println!("Hello, from function!");
}
// for passing the parameters
fn second_fn(x: i32){
   println!("The value of x is: {}", x);
}
// statement  where function return or print something 
// multiple parameters
fn third_fn(x: i32, y: char){
    println!("The value of x is: {x} , and the value of y is :{y}");
}
// expression is the function which don't return anything
fn ex(x: i32){
   let y= x + 1;
   println!("The value of y is: {y}");
}

// return value from function 

fn return_value()->i32{
    78+34
}