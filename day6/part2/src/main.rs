use std::fs;
use std::error::Error;
use std::time::SystemTime;


fn main() -> Result<(), Box<dyn Error>>{
    let start_time = SystemTime::now();
    let input: String = fs::read_to_string("input/input.txt")?.parse()?;
    let lines: Vec<_> = input.split("\r\n").collect();
    let time = parse_line(lines[0]);
    let distance = parse_line(lines[1]);

    let mut winning_races = 0;

    let win_distance = distance.1;
    for calc_time in 0..time.1{
        let dist = calc_distance(calc_time, time.1);
        //println!("race: {}, time held: {}, distance traveled {}, win_distance: {}", time.1, calc_time, dist, win_distance);
        if i64::from(dist) > distance.1{
            winning_races += 1;
        }
    }

    println!("{:?}", winning_races);


    let end_time = SystemTime::now();
    let time_taken = end_time.duration_since(start_time).expect("Clock may have gone backwards");
    println!("answer: {} \ntime taken: {:?}", winning_races, time_taken);
    Ok(())
}

fn parse_line(line: &str) -> (String, i64) {
    let mut new_line = line.clone().replace(" ", "");
    let mut items: Vec<&str> = line.split(':').collect::<Vec<&str>>();
    let list_type = items.remove(0);
    //println!("{:?}", items[0].replace(" ",""));

    let value = items[0].replace(" ","").parse::<i64>().unwrap();

    return (list_type.to_string(), value);
}

fn calc_distance(hold_time: i64, total_time: i64) -> i64{
    let run_time = total_time - hold_time;
    if hold_time == total_time || hold_time == 0 {
        return 0;
    }
    
    return hold_time * run_time;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calc1(){
        let result = calc_distance(0, 7);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_distance_calc2(){
        let result = calc_distance(1, 7);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_distance_calc3(){
        let result = calc_distance(2, 7);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_distance_calc4(){
        let result = calc_distance(2, 7);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_distance_calc5(){
        let result = calc_distance(3, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_distance_calc6(){
        let result = calc_distance(4, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_distance_calc7(){
        let result = calc_distance(5, 7);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_distance_calc8(){
        let result = calc_distance(6, 7);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_distance_calc9(){
        let result = calc_distance(7, 7);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_distance_calc10(){
        let result = calc_distance(7, 7);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parse_line1(){
        let result = parse_line("Time:      7  15   30");
        println!("{:?}", result);
        let answer = ("Time".to_string(), 71530);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_parse_line2(){
        let result = parse_line("Distance:  9  40  200");
        let answer = ("Distance".to_string(), 940200);
        assert_eq!(result, answer);
    }
}