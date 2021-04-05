use std::path::Path;
use super::ColorFormat;

/// Contains the info you get when you load a png
pub struct PngData {
    pub width : u32,
    pub height : u32,
    pub data : Vec<u8>,
    pub format : ColorFormat, 
}

#[derive(Debug)]
pub enum PngLoadError {
    InvalidFormat,
    Decoding(png::DecodingError),
    IO(std::io::Error),
}

/// TODO: MAKE THIS GIVE ERRORS CORRECTLY
pub fn read_png(path : &Path) -> Result<PngData, PngLoadError> {
    
    use std::fs::File;

    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => return Err(PngLoadError::IO(error))
    };

    let decoder = png::Decoder::new(file);

    let (info, mut reader) = match decoder.read_info() {
        Ok(ir) => ir,
        Err(error) => return Err(PngLoadError::Decoding(error))
    };

    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    match reader.next_frame(&mut buf) {
        Ok(_) => (),
        Err(error) => return Err(PngLoadError::Decoding(error))
    };

    let format = match info.color_type {
        png::ColorType::RGB => ColorFormat::RGB,
        png::ColorType::RGBA => ColorFormat::RGBA,
        _ => {return Err(PngLoadError::InvalidFormat)}
    };
    
    Ok(PngData {width : info.width, height : info.height, data : buf, format})
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn read_png() {
        use std::fs::File;

        let decoder = png::Decoder::new(File::open("textures/atlas.png").unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; info.buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        reader.next_frame(&mut buf).unwrap();
    }

    #[test]
    #[ignore]
    fn write_png() {
        // For reading and opening files
        use std::path::Path;
        use std::fs::File;
        use std::io::BufWriter;

        let path = Path::new(r"textures/write.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 2, 1); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
        writer.write_image_data(&data).unwrap(); // Save
    }
}