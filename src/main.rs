use std::env;
use std::result;
use std::collections;


fn main() {
    // get input from commandline arguments
    match process_args(env::args()) {
        Ok(params) => path_calc(params),
        Err(message) => {
            println!("Error in arguments: {}", message);
            usage();
        }        
    }
}


fn usage() {
    println!("Usage: cantstopcalc [-b BLOCKED] [-c CHOSEN] ...");
    println!("Where:");
    println!("\tBLOCKED - numbers of paths which are blocked");
    println!("\tCHOSEN - numbers of paths which are picked by the player");
}


struct CalcParams {
    blocked: collections::HashSet<u8>,
    chosen: collections::HashSet<u8>
}


enum ArgMode {
    Blocked,
    Chosen
}


fn process_args(args: env::Args) -> result::Result<CalcParams, std::string::String> {
    let mut mode = None;
    let mut ret = CalcParams {
        blocked: collections::HashSet::new(),
        chosen: collections::HashSet::new()
    };
    // process arguments sequentially
    for arg in args.skip(1) {
        // check for mode change
        if arg.chars().nth(0) == Some('-') {
            match &arg[..] {
                "-b" => mode = Some(ArgMode::Blocked),
                "-c" => mode = Some(ArgMode::Chosen),
                _ => return Err(format!("unrecognized option: {}", arg))
            }
            continue;   
        }
        match &mode {
            Some(mode) => {
                let path: u8 = match arg.parse() {
                    Ok(num) => num,
                    Err(_) => return Err(format!("failed to parse path number: {}", arg))
                };
                // check if path is is range
                if path < 2 || path > 12 {
                    return Err(format!("path number outside range: {}", path));
                }
                match &mode {
                    ArgMode::Blocked => { ret.blocked.insert(path); () }
                    ArgMode::Chosen => { ret.chosen.insert(path); () }
                }
            }
            None => return Err(format!("specify an option first instead of {}", arg))
        }
    }
    if ret.chosen.len() > 3 {
        return Err(format!("too much chosen paths: {}", ret.chosen.len()));
    }
    return Ok(ret);
}


fn dice_values(mut n: u32) -> [u8; 4] {
    let mut ret: [u8; 4] = [0; 4];
    for i in 0 .. 4 {
        ret[i] = ((n % 6) + 1) as u8;
        n = n / 6;
    }
    return ret;
}


fn dice_sums(d: [u8; 4]) -> [(u8, u8); 3] {
    return [
        (d[0] + d[1], d[2] + d[3]),
        (d[0] + d[2], d[1] + d[3]),
        (d[0] + d[3], d[1] + d[2])
    ];
}

 
fn path_calc(params: CalcParams) {
    let mut fail_chance = 0; // # dice states resulting in a lost round

    // process all possible dice results
    for n in 0 .. (6 * 6 * 6 * 6) {
        let d = dice_values(n);
        let sums = dice_sums(d); // 3 possible dice partitions
        let mut has_choice = false;

        for (sum_1, sum_2) in sums.iter() {
            let sum_1_blocked = params.blocked.contains(sum_1);
            let sum_2_blocked = params.blocked.contains(sum_2);

            let both_can_go = params.chosen.union(
                &[*sum_1, *sum_2].iter().cloned().collect()).count() <= 3;
            let sum_1_can_go = params.chosen.union(
                &[*sum_1].iter().cloned().collect()).count() <= 3;
            let sum_2_can_go = params.chosen.union(
                &[*sum_2].iter().cloned().collect()).count() <= 3;
            
            // can progress no both sums
            if !sum_1_blocked && !sum_2_blocked && both_can_go {
                has_choice = true;
                continue;
            }

            // can progress on 1st sum only
            if !sum_1_blocked && sum_1_can_go {
                has_choice = true;
            }
            // can progress on 2nd sum only
            if !sum_2_blocked && sum_2_can_go {
                has_choice = true;
            }
        }
        
        if !has_choice {
            // no choice in any of the partitions
            fail_chance += 1;
        }
    }
    
    println!("Failing states count: {}", fail_chance);
    println!("Failing state probability: {:.3}", fail_chance as f64 / ((6 * 6 * 6 * 6) as f64));
}

