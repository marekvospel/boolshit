pub use boolshit::Boolshit;

pub trait IsInternal {
    fn is_internal(&self) -> bool;
}
pub trait ShouldLog {
    fn should_log(&self) -> bool;
}
pub trait StatusCode {
    fn status_code(&self) -> usize;
}

#[derive(Boolshit)]
#[boolshit(IsInternal, is_internal, false)]
#[boolshit(ShouldLog, should_log, true)]
#[boolshit(StatusCode, status_code, 400)]
pub enum MyError {
    #[boolshit(is_internal = true)]
    #[boolshit(status_code = 500)]
    IoError,
    #[boolshit(should_log = false)]
    #[boolshit(status_code = 401)]
    IncorrectPassword,
    #[boolshit(is_internal = transparent)]
    CustomError(ErrorA),
}

pub struct ErrorA {}

impl IsInternal for ErrorA {
    fn is_internal(&self) -> bool {
        true
    }
}

#[test]
fn it_should_implement_bool() {
    let none = MyError::IoError;

    assert!(none.is_internal());
    let none = MyError::IncorrectPassword;

    assert!(!none.is_internal());
}

#[test]
fn it_should_implement_transparent() {
    let transparent = MyError::CustomError(ErrorA {});
    assert!(transparent.is_internal());
}

#[test]
fn it_should_implement_usize() {
    let none = MyError::IoError;

    assert_eq!(none.status_code(), 500);
    let none = MyError::IncorrectPassword;

    assert_eq!(none.status_code(), 401);
}
