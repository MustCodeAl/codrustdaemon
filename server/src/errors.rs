#[derive(Debug)]
enum CLiError {
    GenericError = 1,
    NotImplementedError = 2,
    BinaryNotFound = 3,
}


#[derive(Debug)]
struct ErrorResponse  {
    code: CLiError,
    desc: String,
}