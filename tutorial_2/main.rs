fn main () {

   /* Ten element array initialized to zeros. */
   let mut xs : [i64; 10] = [0; 10];

   /* Fill the array with an incrementing pattern */
   for n in 0..xs.len() {
      xs[n] = n as i64; /* cast the usize as an i64 */
   }

   /* Print the array in its entirety */
   for n in 0..xs.len() {
      println!("I see the value {}!", xs[n]);
   }

   println!("Hello, world!");
}
