use std::{fs::File, io, io::{BufRead}, path::Path, str::FromStr};

pub struct CsvGrid<T> {
    data : Vec<T>,
    width : usize,
    height : usize
}

impl<T> CsvGrid<T> {
    fn new(data : Vec<T>, width : usize, height : usize) -> Self{
        assert!(data.len() == width*height);
        Self {
            data, width, height
        }
    }

    pub fn data_point(&self, x : usize, y : usize) -> &T {
        if x >= self.width || y >= self.height {
            panic!("Trying to access out of bounds of a CSV grid");
        }

        &self.data[x+y*self.width]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

pub fn read_csv<T : AsRef<Path>, F : FromStr>(path : T) -> std::io::Result<CsvGrid<F>> {
    let file = File::open(path)?;

    let reader = io::BufReader::new(&file);

    let mut data = Vec::new();
    let mut iter = reader.lines();
    let line1 = iter.next().unwrap()?;
    for number in line1.split(',') {
        println!("A number!");
        match number.parse::<F>() {
            Ok(d) => {data.push(d);}
            Err(_) => { panic!("Failed to load CSV due to a parsing error!") }
        }
    }
    let width = line1.chars().filter(|x| *x == ',').count()+1;
    let mut height = 1;
    for line in iter {
        if let Ok(line) = line {
            height += 1;
            for number in line.split(',') {
                match number.parse::<F>() {
                    Ok(d) => {data.push(d);}
                    Err(_) => { panic!("Failed to load CSV due to a parsing error!") }
                }
            }
        }
    }
    println!("Width {}, Height {}", width, height);


    Ok(CsvGrid::new(data, width, height))
}