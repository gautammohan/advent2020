use std::collections::{HashMap};
use std::fs;

// If I wasn't trying to speed through this at 1am I would have tried
// to do the obvious DP solution. Luckily Rust is fast enough to make
// my brute force solution work!

// PS don't judge me for all the .clones() and Strings, I was just trying to make
// my code more Pythonic...

fn parse_num_bag(raw: &str) -> (i32, String) {
    if raw == " no other bags." {
        (1, "empty".to_owned())
    } else {
        let splits: Vec<_> = raw.split(" ").collect();
        let n = splits[1].parse::<i32>().expect("could not bagnum");
        let bag = format!("{} {}", splits[2], splits[3]);
        (n, bag)
    }
}

fn parse(s: String) -> HashMap<String, HashMap<String, i32>> {
    let mut bagmap: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in s.lines() {
        let split: Vec<_> = line.split("contain").collect();
        let left: Vec<_> = split[0].split(" ").collect();
        let holder = format!("{} {}", left[0], left[1]);
        let contents = split[1].split(",").collect::<Vec<_>>();
        let mut cmap = HashMap::new();
        for cont in contents {
            let (n, bag) = parse_num_bag(cont);
            cmap.insert(bag, n);
            bagmap.insert(holder.clone(), cmap.clone());
        }
    }
    bagmap
}

fn find(query: &str, bagmap: &HashMap<String, HashMap<String, i32>>) -> bool {
    if query == "empty" {
        return false;
    } else if query == "shiny gold" {
        return true;
    } else {
        let mut found = false;
        for (bag, _) in bagmap.get(query).unwrap() {
            // If I cached my results the recursion tree would be less pretty.
            found |= find(bag, bagmap);
            if found {
                break;
            }
        }
        return found
    }
}

fn count(query: &str, bagmap: &HashMap<String, HashMap<String, i32>>) -> i32 {
    //Surely there must be a better way to do this...
    if bagmap.get(query).unwrap().keys().find(|s| **s == "empty".to_string()).is_some() {
        0
    }
    else {
        let mut sum = 0;
        for (bag, n) in bagmap.get(query).unwrap() {
            // let's recompute each subproblem just to be extra sure we got the right answers
            sum += n * (1 + count(bag, bagmap));
        }
        sum
    }
}

pub fn day7() {
    let f = fs::read_to_string("inputs/day7.txt").expect("file not found");
    let bagmap = parse(f);
    let answer = bagmap.keys().filter(|bag| find(bag, &bagmap)).count() - 1;
    println!("day 7 part 1: {}", answer);
    println!("day 7 part 2: {}", count("shiny gold", &bagmap));
}

#[cfg(test)]
mod test {
    static S: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    use super::*;
    #[test]
    fn parsing() {
        let r = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let map = parse(r.to_string());
        assert_eq!(
            *map.get("light red").unwrap().get("bright white").unwrap(),
            1
        );
        assert_eq!(
            *map.get("light red").unwrap().get("muted yellow").unwrap(),
            2
        );
        let n = "faded blue bags contain no other bags.";
        let map2 = parse(n.to_string());
        assert_eq!(*map2.get("faded blue").unwrap().get("empty").unwrap(), 1);
        let map3 = parse(S.to_string());
        assert_eq!(
            *map3.get("shiny gold").unwrap().get("dark olive").unwrap(),
            1
        );
    }
    #[test]
    fn test_find() {
        let map = parse(S.to_string());
        let found : Vec<_> = map.keys().filter(|bag| find(bag, &map)).collect();
        assert_eq!(found.len()-1, 4);
    }

    #[test]
    fn test_count() {
        let s2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let map = parse(s2.to_string());
        assert_eq!(count("dark blue", &map), 2);
        assert_eq!(count("dark green", &map), 6);
        assert_eq!(count("shiny gold", &map), 126);
    }
}
