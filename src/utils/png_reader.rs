

/// TODO: MAKE THIS GIVE ERRORS CORRECTLY
pub fn read_png(path : &str) -> Vec<u8>{
    
    use std::fs::File;

    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    reader.next_frame(&mut buf).unwrap();

    buf
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