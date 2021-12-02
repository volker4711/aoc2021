use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                forward = forward + val.chars().nth(8).unwrap().to_digit(10).unwrap();
            } else if ch == 'd' {
                depth = depth + val.chars().nth(5).unwrap().to_digit(10).unwrap();
            } else if ch == 'u' {
                depth = depth - val.chars().nth(3).unwrap().to_digit(10).unwrap();
            }
        }

        //day02 1
        result = depth * forward;
        println! ("Res day02 1: {}", result);

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
                let mut tmp: u32 = valround2.chars().nth(8).unwrap().to_digit(10).unwrap();
                forward = forward + tmp;
                depth = depth + (aim * tmp);
            } else if ch == 'd' {
                aim = aim + valround2.chars().nth(5).unwrap().to_digit(10).unwrap();
            } else if ch == 'u' {
                aim = aim - valround2.chars().nth(3).unwrap().to_digit(10).unwrap();
            }
        }

       result = depth * forward;
       println! ("Res day02 2: {}", result);
    } else {
        println! ("Eingabe-Datei kann nicht geöffnet werden");
    }
}
