use itertools::Itertools;
fn main() {
    //Read-File
    let w = "this cat this bat this rat";

    // Word
    let arr_w: Vec<&str> = w.split_whitespace().collect();

    //Unique
    let arr_w = unique(arr_w);

    //Count
    let c = arr_w.len();

    //Final
    println!("{}",c)

}
fn unique(s: Vec<&str>) -> Vec<&str>{
    //let s = vec![s];
    let s: Vec<_> = s.into_iter().unique().collect();
    s
}




#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_unique() {
       let input = vec!["this", "cat" , "this" ,  "bat" ,  "this" ,  "rat"];
       let output = ["this", "cat" , "bat" ,  "rat"];
       assert_eq!(unique(input), output);
   }
}
