fn main() {
    let start = 1;
    let end = 100;
    println!("Start : {} | End : {}",start,end);
    println!("Answer : {}",sum_a2b(start,end));
}
fn sum_a2b(start:u32,end:u32) -> u32{
    // formula : (start+end)(end-start+1)/2
    // sum of 1 to 100 -> start : 1 , end : 100

    // S1 : Start + End
    let s1:u32 = start + end;
    // S2 : End - Start + 1
    let s2:u32 = end - start + 1;
    // S3 : S1 x S2
    let s3 = s1 * s2;
    // S4 : S3 / 2
    let s4 = s3 / 2 ;
    // Output : S4
    s4

}
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_sum_a2b() {
       let input1 = 1 ;
       let input2 =  100;
       let output = 5050;
       assert_eq!(sum_a2b(input1,input2), output);
   }
}
