

pub enum LuxError {

}

pub type Result<T> = std::result::Result<T, LuxError>;

pub trait Light {
    fn test() -> i32;
}