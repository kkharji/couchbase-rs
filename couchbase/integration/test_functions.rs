use crate::tests::*;
use crate::util::TestConfig;
use crate::TestResult;
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;

// Sad panda noises
pub fn tests(config: Arc<TestConfig>) -> Vec<TestFn> {
    vec![
        TestFn::new(
            "test_upsert_get",
            Box::pin(kv::test_upsert_get(config.clone())),
        ),
        TestFn::new(
            "test_upsert_replace_get",
            Box::pin(kv::test_upsert_replace_get(config.clone())),
        ),
        TestFn::new(
            "test_upsert_preserve_expiry",
            Box::pin(kv::test_upsert_preserve_expiry(config.clone())),
        ),
        TestFn::new(
            "test_replace_preserve_expiry",
            Box::pin(kv::test_replace_preserve_expiry(config.clone())),
        ),
        TestFn::new(
            "test_get_with_expiry",
            Box::pin(kv::test_get_with_expiry(config.clone())),
        ),
        TestFn::new("test_query", Box::pin(query::test_query(config.clone()))),
        TestFn::new(
            "test_query_named_params",
            Box::pin(query::test_query_named_params(config.clone())),
        ),
        TestFn::new(
            "test_query_positional_params",
            Box::pin(query::test_query_positional_params(config.clone())),
        ),
        TestFn::new(
            "test_query_prepared",
            Box::pin(query::test_query_prepared(config.clone())),
        ),
        TestFn::new(
            "test_upsert_lookupin",
            Box::pin(subdoc::test_upsert_lookupin(config.clone())),
        ),
        TestFn::new(
            "test_mutatein_basic",
            Box::pin(subdoc::test_mutatein_basic(config.clone())),
        ),
        TestFn::new(
            "test_mutatein_arrays",
            Box::pin(subdoc::test_mutatein_arrays(config.clone())),
        ),
        TestFn::new(
            "test_mutatein_counters",
            Box::pin(subdoc::test_mutatein_counters(config.clone())),
        ),
        TestFn::new(
            "test_mutatein_blank_path_remove",
            Box::pin(subdoc::test_mutatein_blank_path_remove(config.clone())),
        ),
        TestFn::new(
            "test_mutatein_blank_path_get",
            Box::pin(subdoc::test_mutatein_blank_path_get(config.clone())),
        ),
        TestFn::new("test_xattrs", Box::pin(subdoc::test_xattrs(config.clone()))),
        TestFn::new("test_macros", Box::pin(subdoc::test_macros(config.clone()))),
        TestFn::new(
            "test_mutatein_preserve_expiry",
            Box::pin(subdoc::test_mutatein_preserve_expiry(config.clone())),
        ),
    ]
}

pub struct TestFn {
    pub name: String,
    pub func: Pin<Box<dyn Future<Output = TestResult<bool>> + Send + 'static>>,
}

impl TestFn {
    pub fn new(
        name: impl Into<String>,
        func: Pin<Box<dyn Future<Output = TestResult<bool>> + Send + 'static>>,
    ) -> Self {
        Self {
            name: name.into(),
            func,
        }
    }
}