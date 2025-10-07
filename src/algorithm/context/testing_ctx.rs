use testing::entities::test_value::Value;
///
/// Used for testing purposes only
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestingCtx {
    pub mok_user_reply: MokUserReplyTestCtx,
}
///
/// Used for testing purposes only
#[derive(Debug, Clone, PartialEq)]
pub struct MokUserReplyTestCtx {
    pub value: Value,
}