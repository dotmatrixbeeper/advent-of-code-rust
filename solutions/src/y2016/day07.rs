use regex::Regex;

pub fn run(input: &str) {
    println!("======= DAY 7 =======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    let re = Regex::new(r"(?<before>[a-z]*)(\[(?<inner>[a-z]*)\])(?<after>[a-z]*)").unwrap();
    // capture outers and inners 
    // if outer has ABBA and inner doesnt have ABBA then good IPv7
    let ip_count = input
        .lines()
        .filter_map(| line | {
            return is_tls_sequence(&re, line);
        })
        .count();
    println!("TLS supporting IPs: {}", ip_count);
    println!("---------------------");
}

fn is_tls_sequence(regex: &Regex, line: &str) -> Option<()> {
    let mut ip_has_abba = false;
    let mut no_hypernet_abba = true;
    regex
        .captures_iter(line)
        .for_each(|capture| {
            ip_has_abba |= ["before", "after"]
                .iter()
                .any(| group | {
                    capture.name(*group)
                        .is_some_and(| matched | {
                            contains_abba(matched.as_str())
                        })
            });
                
            // assume there are no ABBA in hypernet, and if there is one return false
            // & takes care of the rest.
            no_hypernet_abba &= ["inner"]
                .iter()
                .any(| group | {
                    capture.name(*group)
                        .is_some_and(| matched | {
                            !contains_abba(matched.as_str())
                        })
                })
        });

        (ip_has_abba && no_hypernet_abba).then_some(())
}

fn contains_abba(haystack: &str) -> bool {
    haystack.as_bytes().windows(4)
        .any(| val | val[0] != val[1] && val[0] == val[3] && val[1] == val[2])
}

fn part_2(input: &str) {
    println!("Part 2 --------------");
    let re = Regex::new(r"(?<before>[a-z]*)(\[(?<inner>[a-z]*)\])(?<after>[a-z]*)").unwrap();
    // Now we find ABA sequnece such that it should also be present in the hypernet [] strings
    let ip_count = input
        .lines()
        .filter_map(| line | {
            return is_ssl_sequence(&re, line);
        })
        .count();
    println!("SSL supporting IPs: {}", ip_count);
    println!("---------------------");
}

fn is_ssl_sequence(regex: &Regex, line: &str) -> Option<()> {
    let mut hypernet = String::new();
    let mut supernet = String::new();
    regex
        .captures_iter(line)
        .for_each(|capture| {
            // each capture will contain all three "before", "after" and "inner" groups
            // do not assume like me that it iterates over the groups in the for_each
            // iteration is over each capture.
            // I am stupid.
            if let Some(val) = capture.name("before") {
                supernet.push_str(val.as_str());
            } 
            
            if let Some(val) = capture.name("after") {
                supernet.push_str(val.as_str());
            } 
            
            if let Some(val) = capture.name("inner") {
                hypernet.push_str(val.as_str());
            }
        });
        
    if valid_aba(&hypernet, &&supernet) {
        Some(())
    } else {
        None
    }

}

fn valid_aba(hypernet: &str, outers: &str) -> bool {
    outers.as_bytes()
        .windows(3)
        .any(| pattern | {
            let pattern_vec = pattern.to_vec();
            let mut bab = String::new();
            bab.push(pattern_vec[1] as char);
            bab.push(pattern_vec[0] as char);
            bab.push(pattern_vec[1] as char);
            return pattern[0] == pattern[2] && pattern[0] != pattern[1] && hypernet.contains(&bab)
        })
}