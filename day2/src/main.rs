use std::fs;
#[derive(PartialEq, Eq)]
enum Change{
    INCREASE,
    DECREASE,
    None
}
fn main() {
    let data: String = fs::read_to_string("data.txt").unwrap();
    let rowiter = data.split('\n');
    let mut numsafereports = 0; 
    let mut countrow = 0;
    for row in rowiter{
        countrow+=1;
        let report : Vec<i32> = row.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let safereport = analyzereport(report, true);
        if safereport{
            println!("Safe report line: {}",countrow);
            numsafereports+=1; 
        }
    }
    println!("Number of safe reports: {}",numsafereports);
}

// if allowrecursion{
//     return analyzereport({let mut report = report.clone(); report.remove(index); report }, false) || 
// analyzereport({let mut report = report.clone(); report.remove(index-1); report }, false);
// }

fn analyzereport(report:Vec<i32>, allowrecursion: bool) -> bool{
    let mut previouschange = Change::None;
    for index in 1..report.len(){
        let difference = report[index] -report[index-1];
        if  difference.abs() > 3 || difference.abs() == 0 {
//             if allowrecursion{
//     return analyzereport({let mut report = report.clone(); report.remove(index); report }, false) || 
// analyzereport({let mut report = report.clone(); report.remove(index-1); report }, false);
// }
            if allowrecursion{
                let mut bol = false; 
                for x in 0..report.len(){
                    bol = bol || analyzereport({let mut report = report.clone(); report.remove(x); report }, false)
                }
                return bol; 
            }
            return false;
        }
        match difference.cmp(&0){
            std::cmp::Ordering::Less => {
                if previouschange == Change::INCREASE{
//                     if allowrecursion{
//     return analyzereport({let mut report = report.clone(); report.remove(index); report }, false) || 
//  analyzereport({let mut report = report.clone(); report.remove(index-1); report }, false);
//  }
                    if allowrecursion{
                        let mut bol = false; 
                        for x in 0..report.len(){
                            bol = bol || analyzereport({let mut report = report.clone(); report.remove(x); report }, false)
                        }
                        return bol; 
                    }
                    return false;
                } 
                previouschange = Change::DECREASE
                

            },
            std::cmp::Ordering::Greater => {
                if previouschange == Change::DECREASE{
//                     if allowrecursion{
//     return analyzereport({let mut report = report.clone(); report.remove(index); report }, false) || 
// analyzereport({let mut report = report.clone(); report.remove(index-1); report }, false);
// }
                    if allowrecursion{
                        let mut bol = false; 
                        for x in 0..report.len(){
                            bol = bol || analyzereport({let mut report = report.clone(); report.remove(x); report }, false)
                        }
                        return bol; 
                    }
                    return false;
                } 
                previouschange = Change::INCREASE

            },
            std::cmp::Ordering::Equal => unreachable!("Condition should lead to break in if clause before match")
        }
    }
    return true;

}

fn part1(){
    let data: String = fs::read_to_string("data.txt").unwrap();
let rowiter = data.split('\n');
let mut numsafereports = 0; 
for row in rowiter{
    let report : Vec<i32> = row.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut previouschange = Change::None;
    let mut safereport = true;
    for index in 1..report.len(){
        let difference = report[index] -report[index-1];
        if  difference.abs() > 3 || difference.abs() == 0 {
            safereport = false;
            break;
        }
        match difference.cmp(&0){
            std::cmp::Ordering::Less => {
                if previouschange == Change::INCREASE{
                    safereport = false;
                    break;
                } 
                previouschange = Change::DECREASE
                

            },
            std::cmp::Ordering::Greater => {
                if previouschange == Change::DECREASE{
                    safereport = false;
                    break;
                } 
                previouschange = Change::INCREASE

            },
            std::cmp::Ordering::Equal => unreachable!("Condition should lead to break in if clause before match")
        }
    }
    if safereport{
        numsafereports+=1; 
    }
}
println!("Number of safe reports: {}",numsafereports);
}
