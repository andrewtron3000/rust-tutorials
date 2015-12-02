fn main()
{
    println!("Hello, world!");
}

#[test]
fn test1()
{
   /* Google search: "rust too many linked lists" */
   
   let mut xs = Vec::new();

   /*
    * Add a few elements to the vector.
    */
   xs.push(10);
   xs.push(20);
   xs.push(30);

   /*
    * Check to make sure that indexing works OK.
    */
   assert_eq!(10, xs[0]);
   assert_eq!(20, xs[1]);
   assert_eq!(30, xs[2]);

   /*
    * Check to make sure the pops work OK.
    */
   assert_eq!(30, xs.pop().unwrap());
   assert_eq!(20, xs.pop().unwrap());
   assert_eq!(10, xs.pop().unwrap());

}
