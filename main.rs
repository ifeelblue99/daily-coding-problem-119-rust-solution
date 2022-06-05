use std::cmp;

fn main() {
  let data = Array::new(vec![ vec![1, 5], vec![9, 10], vec![0, 5], vec![6, 13]]);

  if let Ok(res) = data.get_range() {
    println!("range: [{}, {}]", res[0], res[1]);
  }
}
struct Array {
  data: Vec<Vec<i32>>
}
impl Array {
  fn new(arr: Vec<Vec<i32>>) -> Array {
    Array { data: arr}
  }
  fn get_range(&self) -> Result<[i32;2], &'static str> {
      let mut min_value: i32 = 0;
      let mut max_value: i32 = 0;

      for indx in 0..self.data.len() {
        min_value = cmp::min(min_value, self.data[indx][0]);
        max_value = cmp::max(max_value, self.data[indx][1]);
      }
    
    Ok([min_value, max_value])
  }  
}
