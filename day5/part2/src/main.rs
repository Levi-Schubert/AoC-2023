use std::fs;
use std::error::Error;
use std::time::SystemTime;


fn main() -> Result<(), Box<dyn Error>>{
    let start_time = SystemTime::now();
    let input: String = fs::read_to_string("input/input.txt")?.parse()?;
    let lines: Vec<&str> = input.split("\n").collect();

    let mut seeds: Vec<Seed> = Vec::<Seed>::new();
    let mut tables: Vec<LookupTable> = Vec::<LookupTable>::new();
    let mut current_table = LookupTable{
        source: LookupType::NullTable,
        destination: LookupType::NullTable,
        ranges: Vec::<Range>::new()
    };


    for line in lines{
        if line.contains("seeds:"){
            let seed_strings: Vec<&str> = line.split(" ").collect();
            let mut val: i64 = -1;
            for item in seed_strings{
                if item != "seeds:"{
                    if val == -1{
                        val = item.parse::<i64>().unwrap();
                    }else{
                        seeds.push(Seed{
                            initial_value: val.clone(),
                            range: item.parse::<i64>().unwrap(),
                            current_stage: LookupType::Seed,
                            current_value: val
                        });
                        val = -1;
                    }
                }
            }
        }else{
            let split: Vec<&str> = line.split(" ").collect();
            if split.len() == 1 && split[0] == ""{
                if current_table.source != LookupType::NullTable{
                    tables.push(current_table.clone());
                }
            }else{

                if split[1] == "map:" {
                    let map_words: Vec<&str> = split[0].split("-").collect();
                    current_table = LookupTable{
                        source: find_enum_for_str(map_words[0]),
                        destination: find_enum_for_str(map_words[2]),
                        ranges: Vec::<Range>::new()
                    }
                }else{
                    //create ranges
                    //println!("{:?}", split );

                    current_table.ranges.push(Range{
                        dest_range_start: split[0].parse::<i64>().unwrap(),
                        src_range_start: split[1].parse::<i64>().unwrap(),
                        range_length: split[2].parse::<i64>().unwrap(),
                    });
                }
            }
        }
    }
    tables.push(current_table.clone());

    //println!("{:?}", seeds);
    //println!("{:?}", tables);

    let mut result_seed = Seed{
                            initial_value: 0,
                            range: 0,
                            current_stage: LookupType::NullTable,
                            current_value: i64::MAX
                        };

    let mut full_seeds = Vec::<Seed>::new();

    for seed in &seeds{
        //println!("{:?}", seed);
        for i in 0..seed.range{
            let mut item = seed.clone();
            item.initial_value += i;
            item.current_value += i;
            full_seeds.push(item.clone());
            //println!("{:?}", curr_seed);
            while item.current_stage != LookupType::Location {
            //println!("{:?}", item);

                let table_index: usize = tables.iter().position(|t| t.source == item.current_stage).unwrap();
                let table = &tables[table_index];
                item.current_value = table.get_destination_from_source(item.current_value);
                item.current_stage = table.destination.clone();
                //println!("{:?}", curr_seed);

            }
            if item.current_value < result_seed.current_value{
                println!("lowest updated: {}", item.current_value);
                result_seed = item.clone();
            }
        }
    }



    //println!("{:?}", result_seeds);

    let end_time = SystemTime::now();
    let time_taken = end_time.duration_since(start_time).expect("Clock may have gone backwards");
    println!("answer: {:?} \ntime taken: {:?}", result_seed, time_taken);
    Ok(())
}

fn find_enum_for_str(str: &str) -> LookupType{
    match str {
        "seed" => LookupType::Seed,
        "soil" => LookupType::Soil,
        "fertilizer" => LookupType::Fertilizer,
        "water" => LookupType::Water,
        "light" => LookupType::Light,
        "temperature" => LookupType::Temperature,
        "humidity" => LookupType::Humidity,
        "location" => LookupType::Location,
        _ => LookupType::ParseError
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LookupType{
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
    ParseError,
    NullTable
}

#[derive(Debug, Clone)]
struct LookupTable{
    source: LookupType,
    destination: LookupType,
    ranges: Vec<Range>
}

impl LookupTable{
    fn get_destination_from_source(&self, n: i64) -> i64{
        for range in &self.ranges{
            if range.is_in_range(n){
                return range.get_destination(n);
            }
        }
        n
    }
}


#[derive(Debug, Clone)]
struct Range{
    dest_range_start: i64,
    src_range_start: i64,
    range_length: i64
}

impl Range{
    fn is_in_range(&self, n:i64) -> bool{
        n < self.src_range_start + self.range_length && n >= self.src_range_start
        //n > self.dest_range_start && n < (self.dest_range_start + (self.range_length - 1)) && n > self.src_range_start && n < (self.src_range_start + (self.range_length - 1))
    }

    fn get_destination(&self, n:i64) -> i64 {
        self.dest_range_start + (n - self.src_range_start)

    }
}

#[derive(Debug, Clone)]
struct Seed{
    initial_value: i64,
    range: i64,
    current_stage: LookupType,
    current_value: i64
}


fn process_line(line: &str) -> String {
    return "to-do".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one(){
        let result = process_line("");
        assert_eq!(result, "result");
    }
}