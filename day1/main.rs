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

    if let Ok(lines) = read_lines("C:/Users/user/Documents/dayone_input.txt")
    {
        let mut i: usize = 0;
        let mut y: i32 = 0;
        let mut inhalt = String::new();
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
            let aaa: i32 = v[i].parse().unwrap();
            let aaa2: i32 = v[i+1].parse().unwrap();
            let aaa3: i32 = v[i+2].parse().unwrap();

            i = i+1;

            if i+2 == v.len() { break;}

            let bbb: i32 = v[i].parse().unwrap();
            let bbb2: i32 = v[i+1].parse().unwrap();
            let bbb3: i32 = v[i+2].parse().unwrap();

            if ((aaa + aaa2 + aaa3) < (bbb + bbb2 + bbb3))
            {
                y += 1;
            }
        }

        let gnarf: bool = true;

    } else {
        println! ("Eingabe-Datei kann nicht geöffnet werden");
    }
}


