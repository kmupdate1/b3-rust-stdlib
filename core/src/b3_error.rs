/// 論理破綻時のエラー通知
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum B3Error {
    Invalid,
    Internal,
}
