
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn roedel_vec(v: &mut Vec<String>, whichbittolookat: usize, fuu: bool) -> Vec<String>
{
    let mut sumbit1: u32 = 0;

    for val in v.iter()
    {
        let bit1 = val.chars().nth(whichbittolookat).unwrap();
        sumbit1 = sumbit1 + bit1.to_digit(10).unwrap();
    }


    let mut lengthofthismess: u32 = 0;
    lengthofthismess = v.len().try_into().unwrap();
    let mut lengthofthismessnew: f32  = lengthofthismess as f32 / 2.0;


    let mut bmehr1ser: bool = lengthofthismessnew <= sumbit1 as f32;
    
    let mut tmpvec: Vec<String> = Vec::new();

    for valreduze in v.iter()
    {
        if((fuu && bmehr1ser) || (!fuu && !bmehr1ser))
        {
            if valreduze.chars().nth(whichbittolookat).unwrap() == '1'
            {
                tmpvec.push(valreduze.to_string());
            }
        }

        if((fuu && !bmehr1ser) || (!fuu && bmehr1ser))
        {
            if valreduze.chars().nth(whichbittolookat).unwrap() == '0'
            {
                tmpvec.push(valreduze.to_string());
            }
        }
    }

    return tmpvec;
}

fn main()
{
    if let Ok(lines) = read_lines("/Users/user/Documents/day3input.txt")
    {
        let mut v = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines
        {
            if let Ok(zeile) = line
            {
                v.push(zeile);
            }
        }

        let mut resv: Vec<String> = v.clone();
        for n in 0..=12 {
            if resv.len() != 1 {
                resv = roedel_vec(&mut resv, n, true);
            } else {
                break;
            }
        }

        let mut bin_idx = resv.first().cloned().unwrap();
        let mut ox = isize::from_str_radix(&bin_idx, 2).unwrap();

        resv = v.clone();
        for h in 0..=12 {
            if resv.len() != 1 {
                resv = roedel_vec(&mut resv, h, false);
            } else {
                break;
            }
        }

        bin_idx = resv.first().cloned().unwrap();
        let mut co = isize::from_str_radix(&bin_idx, 2).unwrap();

        println! ("ox: {}", ox);
        println! ("co: {}", co);
        println! ("res: {}", ox * co);

    } else {
        println! ("Eingabe-Datei kann nicht geöffnet werden");
    }
}
