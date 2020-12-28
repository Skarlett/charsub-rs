
#[derive(Debug)]
pub enum Error {
    BadInput(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, x: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(x, "{:?}", &self);
        Ok(())
    }
}
