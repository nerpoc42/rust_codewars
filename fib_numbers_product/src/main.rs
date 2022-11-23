// https://www.codewars.com/kata/5541f58a944b85ce6d00006a

fn product_fib(prod: u64) -> (u64, u64, bool) {
   let (mut a, mut b, mut x) = (0, 1, 0);

   while x < prod {
       (a, b) = (b, a+b);
       x = a * b;
   }

   (a, b, x == prod)
}

fn main() {
    println!("{:?}", product_fib(5895));
}
