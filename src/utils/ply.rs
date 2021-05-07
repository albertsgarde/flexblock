<<<<<<< HEAD
use std::cmp::PartialEq;
use std::fs::File;
=======
use std::{fs::{File}};
>>>>>>> 43-load-ply-point-clouds
use std::io::BufReader;
use std::io::{self, Read, Write};
use std::path::Path;
use std::slice;
<<<<<<< HEAD
=======
use std::cmp::PartialEq;
use std::str::FromStr;
use std::default::Default;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
>>>>>>> 43-load-ply-point-clouds

#[derive(PartialEq)]
enum PlyTypes {
    FLOAT,
    UCHAR,
}

impl PlyTypes {
<<<<<<< HEAD
    fn type_from_string(s: &str) -> Result<PlyTypes, &str> {
        match s {
            "float" => Ok(PlyTypes::FLOAT),
            "uchar" => Ok(PlyTypes::UCHAR),
            _ => Err("Unsupported type!"),
=======
    fn type_from_string(s : &str) -> Result<PlyTypes, PlyError> {
        match s {
            "float" => Ok(PlyTypes::FLOAT),
            "uchar" => Ok(PlyTypes::UCHAR),
            _ => Err(PlyError::UnsupportedType(String::from(s)))
>>>>>>> 43-load-ply-point-clouds
        }
    }

    fn size(&self) -> usize {
        match self {
            PlyTypes::FLOAT => 4,
            PlyTypes::UCHAR => 1,
        }
    }
}

impl std::fmt::Display for PlyTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) ->  std::fmt::Result {
        write!(f, "{}", match self {
            PlyTypes::FLOAT => "float",
            PlyTypes::UCHAR => "uchar"
        })
    }
}

#[repr(C)]
#[derive(Default, Debug, PartialEq)]
pub struct VertAligned {
    x: f32,
    y: f32,
    z: f32,
    r: f32,
    g: f32,
    b: f32,
    nx: f32,
    ny: f32,
    nz: f32,
}

<<<<<<< HEAD
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
=======
fn fill_from_f32<T : ByteOrder>(out : &mut f32, inp : &[u8], loc : &mut usize) {
    *out = T::read_f32(&inp[*loc..*loc+4]);
    *loc += 4;
}
fn fill_from_u8(out : &mut f32, inp : &[u8], loc : &mut usize) {
    *out = inp[*loc] as f32/ 255 as f32;
    *loc += 1;
}

impl VertAligned {
    fn from_bytes<T : ByteOrder>(bytes : &[u8], locations : &[VertLocations]) -> VertAligned {
        let mut res = VertAligned::default();
        let mut loc = 0;
        for location in locations {
            match location {
                VertLocations::X => fill_from_f32::<T>(&mut res.x, bytes, &mut loc),
                VertLocations::Y => fill_from_f32::<T>(&mut res.y, bytes, &mut loc),
                VertLocations::Z => fill_from_f32::<T>(&mut res.z, bytes, &mut loc),
                VertLocations::R => fill_from_u8(&mut res.r, bytes, &mut loc),
                VertLocations::G => fill_from_u8(&mut res.g, bytes, &mut loc),
                VertLocations::B => fill_from_u8(&mut res.b, bytes, &mut loc),
                VertLocations::NX => fill_from_f32::<T>(&mut res.nx, bytes, &mut loc),
                VertLocations::NY => fill_from_f32::<T>(&mut res.ny, bytes, &mut loc),
                VertLocations::NZ => fill_from_f32::<T>(&mut res.nz, bytes, &mut loc)
            }
        }
        res
    }
}

enum VertLocations {
    X,Y,Z,R,G,B,NX,NY,NZ
}

impl FromStr for VertLocations {
    type Err = PlyError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(VertLocations::X),
            "y" => Ok(VertLocations::Y),
            "z" => Ok(VertLocations::Z),
            "red" => Ok(VertLocations::R),
            "green" => Ok(VertLocations::G),
            "blue" => Ok(VertLocations::B),
            "r" => Ok(VertLocations::R),
            "g" => Ok(VertLocations::G),
            "b" => Ok(VertLocations::B),
            "nx" => Ok(VertLocations::NX),
            "ny" => Ok(VertLocations::NY),
            "nz" => Ok(VertLocations::NZ),
            _ => Err(PlyError::UnknownLocation(String::from(s))),
        }
>>>>>>> 43-load-ply-point-clouds
    }
}

#[derive(Debug)]
pub enum PlyError {
    IOError(io::Error),
    UnknownLocation(String),
    UnsupportedElement(String),
    UnknownHeaderElement(String),
    DataLeftInFile { bytes: usize },
    UnsupportedType(String),
    WrongTypeAmount,
<<<<<<< HEAD
    TypesNotMatching { type_index: usize },
=======
    ParseError(std::num::ParseIntError),
    MalformedProperty(String),
    MissingElement(String)
>>>>>>> 43-load-ply-point-clouds
}

impl From<io::Error> for PlyError {
    fn from(e: io::Error) -> PlyError {
        PlyError::IOError(e)
    }
}

impl From<std::num::ParseIntError> for PlyError {
    fn from(e : std::num::ParseIntError) -> PlyError {
        PlyError::ParseError(e)
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

fn bp() {

}

/// Big Endian, Little Endian
/// They have shitty names because of the byteorder package.
enum Endianness {
    BE, LE
}


/// ONLY READS BINARY PLY IN SAME ENDIAN AS SYSTEM!!
/// DOESN'T HANDLE FACES; EDGES
/// T is vertex type
<<<<<<< HEAD
fn read_ply<T: PlyVert, P: AsRef<Path>>(path: P) -> Result<Vec<T>, PlyError> {
=======
fn read_unaligned_ply<P: AsRef<Path>>(path: P) -> Result<Vec<VertAligned>, PlyError> {
>>>>>>> 43-load-ply-point-clouds
    let path = path.as_ref();
    let mut struct_size = 0;
    let mut reader = BufReader::new(File::open(path)?);
    let mut count = 0;
    let mut num_vertices = 0;

    let mut types_in_file = Vec::with_capacity(10);
    let mut locations = Vec::with_capacity(10);
    
    let mut line_number = 0;
    let mut endianness = None;
    loop {
        line_number += 1;
        let (line_buffer, tmp) = read_line(&mut reader)?;
        let line_buffer = line_buffer.trim();
        count += tmp;
        //reader.read_line(&mut line_buffer).unwrap();

        println!("Line: {}", line_buffer);

        if line_buffer.starts_with("end_header") {
            break;
        } else if line_buffer.starts_with("element vertex ") {
            let len = line_buffer.len();
            let line_buffer = String::from(&line_buffer[15..len]);
            println!("Reading {} vertices!", line_buffer);
<<<<<<< HEAD
            num_vertices = line_buffer.parse::<usize>().unwrap();
=======
            bp();
            num_vertices = line_buffer.parse::<usize>()?;

>>>>>>> 43-load-ply-point-clouds
        } else if line_buffer.starts_with("property ") {
            let mut iter = line_buffer.split(' ');
            iter.next();
            if let Some(type_name) = iter.next() {
<<<<<<< HEAD
                match PlyTypes::type_from_string(type_name) {
                    Ok(t) => types_in_file.push(t),
                    Err(_) => return Err(PlyError::UnsupportedType(type_name.to_owned())),
                }
=======
                types_in_file.push(PlyTypes::type_from_string(type_name)?);
                struct_size += types_in_file[types_in_file.len()-1].size();
            } else{
                return Err(PlyError::MalformedProperty(format!("Line {} \"{}\" has a malformed property!", line_number, line_buffer)))
            }
            if let Some(location_name) = iter.next() {
                locations.push(VertLocations::from_str(location_name)?);
            } else {
                return Err(PlyError::MalformedProperty(format!("Line {} \"{}\" has a malformed property!", line_number, line_buffer)))
>>>>>>> 43-load-ply-point-clouds
            }
        } else if line_buffer.starts_with("element face ") {
            return Err(PlyError::UnsupportedElement(format!(
                "read_ply only reads points, does not support faces!"
            )));
        } else if line_buffer.starts_with("element edge ") {
<<<<<<< HEAD
            return Err(PlyError::UnsupportedElement(format!(
                "read_ply only reads points, does not support edges!"
            )));
        } else if !(line_buffer.starts_with("ply")
            || line_buffer.starts_with("format ")
            || line_buffer.starts_with("comment ")
            || line_buffer.starts_with("obj_info "))
        {
            return Err(PlyError::UnknownHeaderElement(line_buffer));
=======
            return Err(PlyError::UnsupportedElement(format!("read_ply only reads points, does not support edges!")))
        } else if line_buffer.starts_with("format ") {
            let mut iter = line_buffer.split(' ');
            iter.next();
            if let Some(end) = iter.next() {
                endianness = match end {
                    "binary_little_endian" => Some(Endianness::LE),
                    "binary_big_endian" => Some(Endianness::BE),
                    _ => return Err(PlyError::UnsupportedElement(format!("Malformed endian element!")))
                };
            } else {
                return Err(PlyError::UnsupportedElement(format!("Malformed endian element!")))
            }
        } else if !(line_buffer.starts_with("ply") ||
            line_buffer.starts_with("comment ") ||
            line_buffer.starts_with("obj_info ")) {
            return Err(PlyError::UnknownHeaderElement(line_buffer.to_owned()));
>>>>>>> 43-load-ply-point-clouds
        }
    }
    println!("Read {} ascii characters!", count);

    if types_in_file.len() < 9 || types_in_file.len() > 10 {
        return Err(PlyError::WrongTypeAmount);
    }
<<<<<<< HEAD
    for i in 0..types_in_file.len() {
        if types_in_file[i] != T::types()[i] {
            return Err(PlyError::TypesNotMatching { type_index: i });
        }
    }
=======
>>>>>>> 43-load-ply-point-clouds

    let mut vert = Vec::with_capacity(num_vertices);
    let mut preverts :Vec<u8> = Vec::with_capacity(num_vertices * struct_size);
    unsafe {
        let buffer =
            slice::from_raw_parts_mut(preverts.as_ptr() as *mut u8, num_vertices * struct_size);
        reader.read_exact(buffer)?;
        preverts.set_len(num_vertices*struct_size);
    }
    if endianness.is_none() {
        return Err(PlyError::MissingElement(format!("No endianness/byteorder element!")));
    }
    let endianness = endianness.unwrap();
    for i in 0..num_vertices {
        match endianness {
            Endianness::BE => vert.push(VertAligned::from_bytes::<BigEndian>(&preverts[i*struct_size..], &locations)),
            Endianness::LE => vert.push(VertAligned::from_bytes::<LittleEndian>(&preverts[i*struct_size..], &locations))
        }
    }


    let mut rest: Vec<u8> = Vec::new();
    reader.read_to_end(&mut rest)?;
    if rest.len() > 0 {
        return Err(PlyError::DataLeftInFile { bytes: rest.len() });
    }

    Ok(vert)
}

<<<<<<< HEAD
=======

fn write_aligned_points<P: AsRef<Path>>(point_cloud : &Vec<VertAligned>, path : P) -> io::Result<()> {
    let mut file = File::create(path)?;

    // Since it used to be a .ply file
    //write!(&mut file, "ply\n")?;
    //write!(&mut file, "format binary_little_endian 1.0\n")?; //TODO: HANDLE ENDIANNESS
    //write!(&mut file, "comment Created by FlexBlock\n")?;
    //write!(&mut file, "obj_info Created by FlexBlock\n")?;
    //write!(&mut file, "element vertex {}\n", point_cloud.len())?;
    //let mut counter=0;
    //for ptype in T::types() {
    //    write!(&mut file, "property {} {}\n", ptype, T::property_name(counter))?;
    //    counter += 1;
    //}
    //write!(&mut file, "end_header\n")?;
    unsafe {
        file.write(&point_cloud.len().to_ne_bytes())?;
        let buffer : &[u8] = std::slice::from_raw_parts(point_cloud.as_ptr() as *const u8, std::mem::size_of::<VertAligned>()*point_cloud.len());
        file.write_all(buffer)?;
    }

    Ok(())
}

pub fn read_aligned_points<P: AsRef<Path>>(path : P)-> io::Result<Vec<VertAligned>> {
    
    let mut reader = BufReader::new(File::open(path)?);
    let mut nbuf = [0u8;8];
    reader.read_exact(&mut nbuf)?;
    let num_vertices = usize::from_ne_bytes(nbuf);
    let struct_size = std::mem::size_of::<VertAligned>();

    let mut verts : Vec<VertAligned> = Vec::with_capacity(num_vertices);
    unsafe {
        let buffer =
            slice::from_raw_parts_mut(verts.as_ptr() as *mut u8, num_vertices * struct_size);
        
            reader.read_exact(buffer)?;
            verts.set_len(num_vertices*struct_size);
    }

    let mut rest = Vec::new();
    reader.read_to_end(&mut rest)?;
    if rest.len() > 0 {
        panic!("Not the correct number of points in aligned point cloud file! Likely an endianness error.");
    }

    Ok(verts)
}

pub fn convert_unaligned_to_aligned<P :AsRef<Path>>(in_path : P, out_path : P) -> Result<(), PlyError> {
    let point_cloud = read_unaligned_ply(in_path)?;

    write_aligned_points(&point_cloud, out_path)?;

    Ok(())
}

>>>>>>> 43-load-ply-point-clouds
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
<<<<<<< HEAD
    fn load_simple() {
        let result = read_ply::<Vert, &str>("graphics/ply/test/minimal.ply");
        if let Err(result) = &result {
            println!("{:?}", result);
        }
        assert!(
            result.is_ok(),
            "read_ply cannot read simple .ply point cloud!"
        );
=======
    #[ignore]
    fn read_minimal_unaligned_ply() -> Result<(), PlyError>{
        use std::time::{Instant};
        let start = Instant::now();
        let vs = read_unaligned_ply::<&str>("graphics/ply/test/minimal.ply")?;
        let first_read = Instant::now();
        println!("Read unaligned ply in {}µs!", (first_read-start).as_micros());

        write_aligned_points(&vs, "graphics/ply/test/minimal.points")?;
        let post_write = Instant::now();
        println!("Wrote aligned points in {}µs!", (post_write-first_read).as_micros());
        
        let vs2 = read_aligned_points("graphics/ply/test/minimal.points")?;
        let second_read = Instant::now();
        println!("Read aligned points in {}µs!", (second_read-post_write).as_micros());
        
        for i in 0..vs.len() {
            if vs[i] != vs2[i] {
                panic!("Vertices don't match!!!");
            }
            if i%100 == 0 {
                print!(".")
            }
        }
        println!("All vertices match! ");
        Ok(())
>>>>>>> 43-load-ply-point-clouds
    }

    #[test]
<<<<<<< HEAD
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
=======
    #[ignore]
    fn read_large_unaligned_ply() -> Result<(), PlyError>{
        use std::time::{Instant};
        let start = Instant::now();
        let vs = read_unaligned_ply::<&str>("graphics/ply/test/large_unaligned.ply")?;
        let first_read = Instant::now();
        println!("Read unaligned ply in {}ms!", (first_read-start).as_millis());

        write_aligned_points(&vs, "graphics/ply/test/large_aligned.points")?;
        let post_write = Instant::now();
        println!("Wrote aligned points in {}ms!", (post_write-first_read).as_millis());
        
        let vs2 = read_aligned_points("graphics/ply/test/large_aligned.points")?;
        let second_read = Instant::now();
        println!("Read aligned points in {}ms!", (second_read-post_write).as_millis());



        for i in 0..vs.len() {
            if vs[i] != vs2[i] {
                panic!("Vertices don't match!!!");
            }
            if i%100000 == 0 {
                print!(".")
>>>>>>> 43-load-ply-point-clouds
            }
        }
        println!("All vertices match!");
        Ok(())
    }

    #[test]
    fn convert_directly() -> Result<(), PlyError>{
        convert_unaligned_to_aligned("graphics/ply/test/minimal.ply", "graphics/ply/test/minimal.points")
    }

    #[test]
<<<<<<< HEAD
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
=======
    fn test_fill() {
        let bytes = [0,0,0,0];
        let mut loc = 0;
        let mut res = 0f32;
        fill_from_f32::<LittleEndian>(&mut res, &bytes, &mut loc);
        println!("{}, {}",res, loc);
        assert_eq!(res, 0f32);
        assert_eq!(loc, 4);
    }
}






// ONLY READS BINARY PLY IN SAME ENDIAN AS SYSTEM!!
// DOESN'T HANDLE FACES; EDGES
// T is vertex type
/*fn read_ply<T : PlyVert, P: AsRef<Path>>(path: P) -> Result<Vec<T>, PlyError> {
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
            let line_buffer = String::from(line_buffer[15..len].trim());
            println!("Reading {} vertices!", line_buffer);
            bp();
            num_vertices = line_buffer.parse::<usize>()?;

        } else if line_buffer.starts_with("property ") {
            let mut iter = line_buffer.split(' ');
            iter.next();
            if let Some(type_name) = iter.next() {
                match PlyTypes::type_from_string(type_name) {
                    Ok(t) => types_in_file.push(t),
                    Err(_) => return Err(PlyError::UnsupportedType(type_name.to_owned()))
                }
            }
        } else if line_buffer.starts_with("element face ") {
            return Err(PlyError::UnsupportedElement(format!("read_ply only reads points, does not support faces!")))
        } else if line_buffer.starts_with("element edge ") {
            return Err(PlyError::UnsupportedElement(format!("read_ply only reads points, does not support edges!")))
        } else if line_buffer.starts_with("format ") {
            let mut iter = line_buffer.split(' ');
        }
         else if !(line_buffer.starts_with("ply") ||
            line_buffer.starts_with("format ") ||
            line_buffer.starts_with("comment ") ||
            line_buffer.starts_with("obj_info ")) {
            return Err(PlyError::UnknownHeaderElement(line_buffer));
        }
    }
    println!("Read {} ascii characters!", count);

    if types_in_file.len() != T::types().len() {
        return Err(PlyError::WrongTypeAmount);
    }
    for i in 0..types_in_file.len() {
        if types_in_file[i] != T::types()[i] {
            return Err(PlyError::TypesNotMatching{type_index : i});
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
        return Err(PlyError::DataLeftInFile {bytes : rest.len()});
    }

    Ok(vert)
}*/
>>>>>>> 43-load-ply-point-clouds
