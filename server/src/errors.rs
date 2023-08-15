enum CLiError {
    GenericError = 1,
    NotImplementedError = 2,
    BinaryNotFound = 3,
}
struct ErrorResponse  {
    code: CLiError,
    desc: String,
}