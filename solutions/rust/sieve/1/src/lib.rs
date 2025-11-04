pub fn primes_up_to(upper_bound:u64) -> Vec<u64> {
   let mut list: Vec<u64> = (0..=upper_bound).collect();
   let mut primes: Vec<u64> = vec![];

   for i in 2..=upper_bound {
      for j in (i*i..=upper_bound).step_by(i as usize) {
         list[j as usize] = 1;
      }
   }
    
   for i in 2..=upper_bound {
      if list[i as usize] != 1 {
         primes.push(list[i as usize]);
      }
   }
    primes
 
}