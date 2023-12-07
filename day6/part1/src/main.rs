use std::fs;
use std::error::Error;
use std::time::SystemTime;


fn main() -> Result<(), Box<dyn Error>>{
    let start_time = SystemTime::now();
    let input: String = fs::read_to_string("input/input.txt")?.parse()?;
    let lines: Vec<_> = input.split("\r\n").collect();
    let time = parse_line(lines[0]);
    let distance = parse_line(lines[1]);

    let mut winning_races = Vec::<i32>::new();

    let mut index = 0;
    for race in time.1{
        let mut win_count = 0;
        let win_distance = distance.1[index];
        for time in 0..race{
            let dist = calc_distance(time, race);
            println!("race: {}, time held: {}, distance traveled {}, win_distance: {}",race, time, dist, win_distance);
            if dist > win_distance{
                win_count += 1;
            }
        }
        index += 1;
        winning_races.push(win_count);
    }

    println!("{:?}", winning_races);

    let mut answer = 1;
    for count in winning_races{
        answer = answer * count;
    }


    let end_time = SystemTime::now();
    let time_taken = end_time.duration_since(start_time).expect("Clock may have gone backwards");
    println!("answer: {} \ntime taken: {:?}", answer, time_taken);
    Ok(())
}

fn parse_line(line: &str) -> (String, Vec<i32>) {
    let mut items: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
    items.retain(|&s| s != "");
    let list_type = items.remove(0).replace(':',"");
    let mut values = Vec::<i32>::new();
    for item in items{
        values.push(item.parse::<i32>().unwrap());
    }

    return (list_type.to_string(), values);
}

fn calc_distance(hold_time: i32, total_time: i32) -> i32{
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
    fn test_parse_line1(){
        let result = parse_line("Time:      7  15   30");
        let answer = ("Time".to_string(), [7 as i32, 15 as i32, 30 as i32].to_vec());
        assert_eq!(result, answer);
    }

    #[test]
    fn test_parse_line2(){
        let result = parse_line("Distance:  9  40  200");
        let answer = ("Distance".to_string(), [9 as i32, 40 as i32, 200 as i32].to_vec());
        assert_eq!(result, answer);
    }
}