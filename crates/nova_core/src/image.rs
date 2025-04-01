use anyhow::Result;

pub struct Image {
    pixels: Vec<u8>,
    size: u64,
    width: u32,
    height: u32
}

impl Image {
    pub(crate) fn new(pixels: Vec<u8>, size: u64, width: u32, height: u32) -> Self {
        Self {
            pixels,
            size,
            width,
            height
        }
    }

    pub fn load(path: &str) -> Result<Self> {
        let image = std::fs::File::open(path)?;

        let decoder = png::Decoder::new(image);
        let mut reader = decoder.read_info()?;
    
        let mut pixels = vec![0; reader.info().raw_bytes()];
        reader.next_frame(&mut pixels)?;
    
        let size = reader.info().raw_bytes() as u64;
        let (width, height) = reader.info().size();

        let s = Self {
            pixels, 
            size,
            width,
            height
        };

        Ok(s)
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}