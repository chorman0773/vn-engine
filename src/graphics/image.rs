use std::path::Path;

pub struct SvgImage(resvg::Tree);

impl SvgImage {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        let data = std::fs::read_to_string(path)?;

        todo!()
    }
}
