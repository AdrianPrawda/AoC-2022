use error::InternalSystemError;

pub mod solution;
pub mod error;

pub trait Solution {
    fn solve(&self, path: String, verbose: bool) -> Result<(), InternalSystemError>;
    fn id(&self) -> u32;
}