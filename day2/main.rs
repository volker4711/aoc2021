use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main()
{
    if let Ok(lines) = read_lines("C:/Users/user/Documents/daytwo_input.txt")
    {
        //let mut i: usize = 0;
        let mut depth: u32 = 0;
        let mut forward: u32 = 0;
        let mut result: u32 = 0;
        let mut aim: u32 = 0;

        let mut v = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines
        {
            if let Ok(zeile) = line
            {
                v.push(zeile);
            }
        }

        for val in v.iter()
        {
            //search for f/d/u
            let ch = val.chars().nth(0).unwrap();

            if ch == 'f' {
                let tmpch: char = val.chars().nth(8).unwrap();
                let mut tmp: u32 = tmpch.to_digit(10).unwrap();
                forward = forward + tmp;
            } else if ch == 'd' {
                let tmpch: char = val.chars().nth(5).unwrap();
                let mut tmp: u32 = tmpch.to_digit(10).unwrap();
                depth = depth + tmp;
            } else if ch == 'u' {
                let tmpch: char = val.chars().nth(3).unwrap();
                let mut tmp: u32 = tmpch.to_digit(10).unwrap();
                depth = depth - tmp;
            }

            //let aaa: i32 = v[i].parse().unwrap();
        }

        //day02 1
        result = depth * forward;
        let mut _gnarf: bool = true;

        //day02 2
        depth = 0;
        forward = 0;
        result = 0;
        aim = 0;

        for valround2 in v.iter()
        {
            //search for f/d/u
            let ch = valround2.chars().nth(0).unwrap();

            if ch == 'f' {
                let tmpch: char = valround2.chars().nth(8).unwrap();
                let mut tmp: u32 = tmpch.to_digit(10).unwrap();
                forward = forward + tmp;
                depth = depth + (aim * tmp);
            } else if ch == 'd' {
                let tmpch: char = valround2.chars().nth(5).unwrap();
                let mut tmp: u32 = tmpch.to_digit(10).unwrap();
                aim = aim + tmp;
            } else if ch == 'u' {
                let tmpch: char = valround2.chars().nth(3).unwrap();
                let mut tmp: u32 = tmpch.to_digit(10).unwrap();
                aim = aim - tmp;
            }
        }

       result = depth * forward;
       _gnarf = true;


    } else {
        println! ("Eingabe-Datei kann nicht geöffnet werden");
    }
}


