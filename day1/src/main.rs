use std::fs;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1(){
    let data: String = fs::read_to_string("data.txt").unwrap();
    let rowiter = data.split('\n');
    let mut lists = [Vec::new(),Vec::new()];
    let mut numrows=0; 
    for row in rowiter{
        numrows+=1;
        let mut numbers = row.split_whitespace();
        lists.iter_mut().for_each(|list| list.push(numbers.next().unwrap().parse::<i32>().unwrap()));
    }
    lists.iter_mut().for_each(|list| list.sort_by(|a,b| a.cmp(b)));
    let mut distance = 0;
    for num in 0..numrows{
       distance = distance + (lists[0][num] - lists[1][num]).abs();
    }
    println!("Distance: {}",distance); 
}

    
fn part2(){
    let data: String = fs::read_to_string("data.txt").unwrap();
    let rowiter = data.split('\n');
    let mut lists = [Vec::new(),Vec::new()];
    for row in rowiter{
        let mut numbers = row.split_whitespace();
        lists.iter_mut().for_each(|list| list.push(numbers.next().unwrap().parse::<usize>().unwrap()));
    }
    let mut similarity = 0;
    let mut hashmap = HashMap::<usize,usize>::new();
    lists[0].iter().for_each(|num| {
        if !hashmap.contains_key(num){
            hashmap.insert(*num, lists[1].iter().filter(|n| (*n).eq(num)).count());
        }
        similarity+=num*hashmap.get(num).unwrap();   
     }); 
    println!("Similarity: {}",similarity);
}
