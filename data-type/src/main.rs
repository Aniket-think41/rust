fn main() {

    /*    scalar data types  ( store single type)  */
// integer , string , boolean , floating , char  ( typoe of the char is also string here ) 

// length of the integer 
// 8 bit , 16, 32, 64, 128 arch   signed(i)ex- i16, unsigned(u)ex- u16

    let no=2;
    println!("The value of no is: {no}");
    let is_sale=true;
    println!("The value of is_sale is: {is_sale}");
    let ch="a";
    println!("The value of ch is: {ch}");
    let str="hello";
    println!("The value of str is: {str}");
    let dec=78.90;
    println!("The value of dec is: {dec}");



/* compound  data types ( where we store multiple types )*/
// array , dictionary, struct


// tuple
   let mut tup=(32,43,56);

   // if you want to print the tuple or any compound data type u have to use default formatter {:?}
   println!("The value of tup is: {:?}",tup);
   // no need to print single element of the tuple
   println!("The value of tup.0 is: {}",tup.0);
   
   tup.0=45;
   println!("The value of tup is: {:?}",tup);

   let tup: (i32,u8,f64)=(32,43,56.9);
   println!("The value of tup is: {:?} ",tup);

// array 
   
   let array=[1,2,3,4,5];
   println!("The value of array is: {:?}",array);
   println!("The value of array[0] is: {}",array[0]);

}




