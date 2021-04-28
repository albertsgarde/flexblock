use std::cmp::PartialEq;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};
use std::path::Path;
use std::slice;

#[derive(PartialEq)]
enum PlyTypes {
    FLOAT,
    UCHAR,
}

impl PlyTypes {
    fn type_from_string(s: &str) -> Result<PlyTypes, &str> {
        match s {
            "float" => Ok(PlyTypes::FLOAT),
            "uchar" => Ok(PlyTypes::UCHAR),
            _ => Err("Unsupported type!"),
        }
    }

    fn size(&self) -> usize {
        match self {
            PlyTypes::FLOAT => 4,
            PlyTypes::UCHAR => 1,
        }
    }
}

trait PlyVert {
    fn types() -> Vec<PlyTypes>;
}

#[repr(packed)]
struct Vert {
    x: f32,
    y: f32,
    z: f32,
    r: u8,
    g: u8,
    b: u8,
    nx: f32,
    ny: f32,
    nz: f32,
}

impl PlyVert for Vert {
    fn types() -> Vec<PlyTypes> {
        return vec![
            PlyTypes::FLOAT,
            PlyTypes::FLOAT,
            PlyTypes::FLOAT,
            PlyTypes::UCHAR,
            PlyTypes::UCHAR,
            PlyTypes::UCHAR,
            PlyTypes::FLOAT,
            PlyTypes::FLOAT,
            PlyTypes::FLOAT,
        ];
    }
}

#[derive(Debug)]
pub enum PlyError {
    IOError(io::Error),
    UnsupportedElement(String),
    UnknownHeaderElement(String),
    DataLeftInFile { bytes: usize },
    UnsupportedType(String),
    WrongTypeAmount,
    TypesNotMatching { type_index: usize },
}

impl From<io::Error> for PlyError {
    fn from(e: io::Error) -> PlyError {
        PlyError::IOError(e)
    }
}

fn read_line(reader: &mut BufReader<std::fs::File>) -> io::Result<(String, usize)> {
    let mut buf: [u8; 1] = [0; 1];
    let mut res = String::new();
    let mut count: usize = 0;
    loop {
        reader.read(&mut buf)?;
        count += 1;
        if buf == *b"\n" {
            break;
        } else {
            res = res + &(buf[0] as char).to_string();
        }
    }
    Ok((res, count))
}

/// ONLY READS BINARY PLY IN SAME ENDIAN AS SYSTEM!!
/// DOESN'T HANDLE FACES; EDGES
/// T is vertex type
fn read_ply<T: PlyVert, P: AsRef<Path>>(path: P) -> Result<Vec<T>, PlyError> {
    let path = path.as_ref();
    let struct_size = ::std::mem::size_of::<T>();
    let mut reader = BufReader::new(File::open(path)?);
    let mut count = 0;
    let mut num_vertices = 0;

    let mut types_in_file = Vec::with_capacity(9);

    loop {
        let (line_buffer, tmp) = read_line(&mut reader)?;
        count += tmp;
        //reader.read_line(&mut line_buffer).unwrap();

        println!("Line: {}", line_buffer);

        if line_buffer.starts_with("end_header") {
            break;
        } else if line_buffer.starts_with("element vertex ") {
            let len = line_buffer.len();
            let line_buffer = String::from(&line_buffer[15..len]);
            println!("Reading {} vertices!", line_buffer);
            num_vertices = line_buffer.parse::<usize>().unwrap();
        } else if line_buffer.starts_with("property ") {
            let mut iter = line_buffer.split(' ');
            iter.next();
            if let Some(type_name) = iter.next() {
                match PlyTypes::type_from_string(type_name) {
                    Ok(t) => types_in_file.push(t),
                    Err(_) => return Err(PlyError::UnsupportedType(type_name.to_owned())),
                }
            }
        } else if line_buffer.starts_with("element face ") {
            return Err(PlyError::UnsupportedElement(format!(
                "read_ply only reads points, does not support faces!"
            )));
        } else if line_buffer.starts_with("element edge ") {
            return Err(PlyError::UnsupportedElement(format!(
                "read_ply only reads points, does not support edges!"
            )));
        } else if !(line_buffer.starts_with("ply")
            || line_buffer.starts_with("format ")
            || line_buffer.starts_with("comment ")
            || line_buffer.starts_with("obj_info "))
        {
            return Err(PlyError::UnknownHeaderElement(line_buffer));
        }
    }
    println!("Read {} ascii characters!", count);

    if types_in_file.len() != T::types().len() {
        return Err(PlyError::WrongTypeAmount);
    }
    for i in 0..types_in_file.len() {
        if types_in_file[i] != T::types()[i] {
            return Err(PlyError::TypesNotMatching { type_index: i });
        }
    }

    let mut vert = Vec::<T>::with_capacity(num_vertices);
    unsafe {
        let buffer =
            slice::from_raw_parts_mut(vert.as_mut_ptr() as *mut u8, num_vertices * struct_size);
        reader.read_exact(buffer)?;
        vert.set_len(num_vertices);
    }

    let mut rest: Vec<u8> = Vec::new();
    reader.read_to_end(&mut rest)?;
    if rest.len() > 0 {
        return Err(PlyError::DataLeftInFile { bytes: rest.len() });
    }

    Ok(vert)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn load_simple() {
        let result = read_ply::<Vert, &str>("graphics/ply/test/minimal.ply");
        if let Err(result) = &result {
            println!("{:?}", result);
        }
        assert!(
            result.is_ok(),
            "read_ply cannot read simple .ply point cloud!"
        );
    }
    #[test]
    fn load_infinite_header() {
        assert!(
            match read_ply::<Vert, &str>("graphics/ply/test/infinite_header.ply") {
                Ok(_) => false,
                Err(e) => match e {
                    PlyError::UnknownHeaderElement(_) => true,
                    _ => false,
                },
            },
            "read_ply doesn't notice when the header is infinite!"
        );
    }
    #[test]
    fn load_size_mismatch_too_long() {
        let result = read_ply::<Vert, &str>("graphics/ply/test/size_mismatch_too_long.ply");
        if let Err(e) = &result {
            println!("{:?}", e);
        }
        assert!(match result {
            Ok(_) => false,
            Err(e) => match e {
                PlyError::DataLeftInFile {bytes : _} => true,
                _ => false
            }
        }, "read_ply accepts a file where the header number of points is less than the actual number of points!");
    }

    #[test]
    fn load_size_mismatch_too_short() {
        let result = read_ply::<Vert, &str>("graphics/ply/test/size_mismatch_too_short.ply");
        assert!(match result {
            Ok(_) => false,
            Err(e) => match e {
                PlyError::IOError(error) => {
                    match error.kind() {
                        std::io::ErrorKind::UnexpectedEof => true,
                        _ => false
                    }
                },
                _ => false
            }
        }, "read_ply accepts a file where the header number of points is greater than the actual number of points!");
    }
    #[test]
    fn types_not_matching() {
        let result = read_ply::<Vert, &str>("graphics/ply/test/types_not_matching.ply");
        assert!(
            match result {
                Ok(_) => false,
                Err(e) => match e {
                    PlyError::TypesNotMatching { type_index: _ } => true,
                    _ => false,
                },
            },
            "read_ply accepts a file with non-matching types!"
        );
    }
    #[test]
    fn too_many_types() {
        let result = read_ply::<Vert, &str>("graphics/ply/test/too_many_types.ply");
        assert!(
            match result {
                Ok(_) => false,
                Err(e) => match e {
                    PlyError::WrongTypeAmount => true,
                    _ => false,
                },
            },
            "read_ply accepts a file with too many types!"
        );
    }
}
