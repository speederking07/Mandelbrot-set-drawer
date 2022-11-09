pub mod bitmap {
    use std::{fmt, path::Path, fs, ops::{Index, IndexMut}};

    /**
     * Struct of 24-bit pixel
     */
    #[derive(Debug, Clone)]
    pub struct Pixel {
        red : u8,
        green : u8,
        blue : u8
    }

    impl Pixel {
        // Useful colors
        pub const BLACK : Pixel = Pixel::new(0, 0, 0);
        pub const WHITE : Pixel = Pixel::new(255, 255, 255);
        pub const RED : Pixel = Pixel::new(255, 0, 0);
        pub const GREEN : Pixel = Pixel::new(0, 255, 0);
        pub const BLUE : Pixel = Pixel::new(0, 0, 255);
    
        /**
         * Create new pixel using RGB
         */
        pub const fn new(red: u8, green: u8, blue: u8) -> Pixel {
            Pixel {red, green, blue}
        }

        /**
         * Sets pixel color using RGB
         */
        pub fn set_rgb<'a>(&'a mut self, red: u8, green: u8, blue: u8) -> &'a mut Pixel{
            self.red = red;
            self.green = green;
            self.blue = blue;
            self
        }
    }

    /**
     * Implementation for pixel displaying
     */
    impl fmt::Display for Pixel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RGB({}, {}, {})", self.red, self.green, self.blue)
        }
    }

    /**
     * Struct of bitmap
     */
    #[derive(Debug)]
    pub struct BitMap {
        width : u32,
        height : u32,
        data : Vec<Vec<Pixel>>,
    }

    /**
     * Converts u16 number as sequance of bytes
     */
    fn u16_to_u8(x : u16) -> Vec<u8> {
        vec![(x % 256) as u8, (x / 256 % 256) as u8]
    }

    /**
     * Converts u32 number as sequance of bytes
     */
    fn u32_to_u8(x : u32) -> Vec<u8> {
        vec![(x % 256) as u8, (x / 256 % 256) as u8, (x / (256 * 256) % 256) as u8, (x / (256 * 256 * 256) % 256) as u8]
    }

    /**
     * Computes required number of bytes to add for current length and offest
     */
    fn offset_to(current : usize, offset: usize) -> usize{
        if current % offset == 0 {
            0
        } else {
            offset - (current % offset)
        }
    }

    impl BitMap {
        /**
         * Creates BitMap out of vector of pixles
         */
        pub fn new(data : Vec<Vec<Pixel>>) -> Result<BitMap, String>{
            let w = data.len() as u32;
            if w == 0{
                return Ok(BitMap {data, width: 0, height: 0})
            }
            let h = data[0].len() as u32;
            for col in data.iter().skip(1){
                if col.len() != h as usize {
                    return Err(String::from("Not a rectangle"));
                }
            }
            return Ok(BitMap {data: data, width: w, height: h})
        }

        /**
         * Creates Bitmap filled with single color
         */
        pub fn new_blank(color: Pixel, width: u32, height: u32) -> BitMap {
            BitMap {data : vec![vec![color; height as usize]; width as usize], width, height}
        }

        /**
         * Creates bitmap with content provided by generator
         */
        pub fn new_from_generator(gen: &dyn Fn(u32, u32) -> Pixel, width: u32, height: u32) -> BitMap {
            let mut bitmap = Self::new_blank(Pixel::WHITE, width, height);
            for x in 0 .. width {
                for y in 0 .. height {
                    bitmap[(x, y)] = gen(x, y)
                }
            }
            bitmap
        }

        /**
         * Draw line form (x1, y1) to (x2, y2)
         */
        pub fn draw_line(&mut self, (x1, y1): (u32, u32), (x2, y2): (u32, u32), color: Pixel) {
            if (x1 as i64 - x2 as i64).abs() > (y1 as i64 - y2 as i64).abs(){
                let (s_x, s_y, f_x, f_y) = if x1 < x2 {
                    (x1, y1, x2, y2)
                } else {
                    (x2, y2, x1, y1)
                };

                let slope = (f_y as f32 - s_y as f32) / (f_x - s_x) as f32;

                for x in s_x ..= f_x{
                    let y = ((x - s_x) as f32 * slope + s_y as f32).round() as u32;
                    self[(x, y)] = color.clone();
                }
            } else {
                let (s_x, s_y, f_x, f_y) = if y1 < y2 {
                    (x1, y1, x2, y2)
                } else {
                    (x2, y2, x1, y1)
                };

                let slope = if y1 != y2 { // to avoid dividing by zero
                    (f_x as f32 - s_x as f32)  / (f_y - s_y) as f32
                } else {
                    0.0
                };

                for y in s_y ..= f_y{
                    let x = ((y - s_y) as f32 * slope + s_x as f32).round() as u32;
                    self[(x, y)] = color.clone();
                }
            }
        }

        /**
         * Creates header for .bmp file
         */
        fn get_header(size : u32) -> Vec<u8> {
            let mut res = vec!['B' as u8, 'M' as u8];
            res.append(&mut u32_to_u8(size)); // size of file
            res.append(&mut u32_to_u8(0));    // restricted
            res.append(&mut u32_to_u8(54));   // offset to data
            res
        }
        
        /**
         * Creates info header for .bmp file
         */
        fn get_info_header(width : u32, height : u32) -> Vec<u8> {
            let mut res = vec![];
            res.append(&mut u32_to_u8(40));     // length of info header
            res.append(&mut u32_to_u8(width));  // width of picture
            res.append(&mut u32_to_u8(height)); // height of picture
            res.append(&mut u16_to_u8(1));      // num of planes
            res.append(&mut u16_to_u8(24));     // bits of colors per pixel
            res.append(&mut u32_to_u8(0));      // compression
            res.append(&mut u32_to_u8(0));      // images size (0 for 0 compression)
            res.append(&mut u32_to_u8(100));    // pixels per meter (x-axis)
            res.append(&mut u32_to_u8(100));    // pixels per meter (y-axis)
            res.append(&mut u32_to_u8(0));      // number of color used (0 for no compression)
            res.append(&mut u32_to_u8(0));      // number of important colors (0 for no compression)
            res
        }

        /**
         * Saves bitmap to specified file
         */
        pub fn save_as_bmp<P>(&self, path: P) -> Result<(), String> where P : AsRef<Path>{
            let mut res = BitMap::get_header((14 + 40 + self.width * self.height * 3) as u32);
            res.append(&mut BitMap::get_info_header(self.width as u32, self.height as u32));
            for y in (0..self.height as usize).rev(){
                for x in 0..self.width as usize{
                    res.push(self.data[x][y].blue);
                    res.push(self.data[x][y].green);
                    res.push(self.data[x][y].red);
                }
                for _ in 0 .. offset_to(self.width as usize * 3, 4){ // adding offset to 4
                    res.push(0);
                }
            }
            match fs::write(path, res) {
                Ok(()) => Ok(()),
                Err(e) => Err(e.to_string())
            }
        }
        
        /**
         * Retruns size of bitmap
         */
        pub fn size(&self) -> (u32, u32) {
            (self.width, self.height)
        }
    }

    /**
     * Implementation of indexing bitmap for accessing single bit
     * bitmap[(1, 1)].set_rgb(0, 255, 0)
     */
    impl Index<(u32, u32)> for BitMap {
        type Output = Pixel;

        fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
            &self.data[x as usize][y as usize]
        }
    }

    impl IndexMut<(u32, u32)> for BitMap {
        fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
            &mut self.data[x as usize][y as usize]
        }
    }
}