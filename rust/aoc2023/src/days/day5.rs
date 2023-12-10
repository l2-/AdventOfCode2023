use std::borrow::BorrowMut;
use std::collections::HashMap;

use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = 5;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));
    let seeds = string_to_ints(&lines[0]);
    
    {

        let mut megaMap: HashMap<i64, Vec<Int3>> = HashMap::new();
        let mut maptype = 0i64;
        megaMap.insert(maptype, vec![]);
        for i in 2..lines.len() {
            let line = &lines[i];
            if line.len() == 0 {
                maptype += 1;
                megaMap.insert(maptype, vec![]);
                continue;
            }
            let ints = string_to_ints(&line);
            if ints.len() == 3 {
                let source = ints[0];
                let destination = ints[1];
                let range = ints[2];
                megaMap
                    .get_mut(&maptype)
                    .unwrap()
                    .push((destination, source, range));
            }
        }
        use std::time::Instant;
        let now = Instant::now();

        let endtype = maptype;

        let mut location = i64::MAX;
        for seed in seeds.iter() {
            let seed = *seed;
            let mut maptype = 0i64;
            let mut current = seed;
            while maptype <= endtype {
                for &(src, dst, range) in megaMap.get(&maptype).unwrap() {
                    if current >= src && current < src + range {
                        let offset = dst - src;
                        let next = current + offset;
                        current = next;
                        break;
                    }
                }

                maptype += 1;
            }
            location = std::cmp::min(location, current);
        }
        println!("Part 1 {}", location);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        let now = Instant::now();
        let mut location = i64::MAX;
        let mut i = 0;
        while i < seeds.len() {
            let seed = seeds[i];
            let range = seeds[i + 1];
            i += 2;
            println!("{} {} {}", i, seed, range);
            
            let mut seed = seed;
            let finish = seed + range;
            while seed < finish {
                let mut maptype = 0i64;
                let mut current = seed;
                let mut maxSkip = i64::MAX;
                while maptype <= endtype {
                    for &(src, dst, range) in megaMap.get(&maptype).unwrap() {
                        if current >= src && current < src + range {
                            let offset = dst - src;
                            let next = current + offset;
                            
                            let last = src + range;
                            let skip = last - current;
                            maxSkip = std::cmp::min(skip, maxSkip);

                            current = next;
                            break;
                        }
                    }

                    maptype += 1;
                }
                location = std::cmp::min(location, current);

                if maxSkip == i64::MAX { maxSkip = 1; }
                seed += maxSkip;
            }            
        }
        println!("Part 2 {}", location);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}
