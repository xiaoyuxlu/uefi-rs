
use super::*;
use linkme::distributed_slice;


#[derive(Debug)]
pub struct TestCase {
    pub name: &'static str,
    pub func: fn() -> (),
    pub should_panic: bool,
}

#[distributed_slice]
pub static TESTCASES: [TestCase] = [..];

pub struct Error;
type Result<T=()> = core::result::Result<T, Error>;

const GREEN_OK: &str = "\x1b[0;32mok\x1b[0m";
const GREEN_SKIP: &str = "\x1b[0;32mSKIP\x1b[0m";
const RED_FAILED: &str = "\x1b[0;31mFAILED\x1b[0m";

pub fn run_unit_tests() -> Result<isize> {

    run_tests(&TESTCASES)
}

fn run_tests(tests: &[TestCase]) -> Result<isize> {
    let test_count = tests.len();

    log::info!(
        "\nrunning {} test{}",
        test_count,
        if test_count == 1 { "" } else { "s" },
    );

    let pass_count = tests.iter().filter(|case| run_one_test(&case)).count();
    let fail_count = (test_count - pass_count) as isize;

    log::info!(
        "\ntest result: {}. {} passed; {} failed\n",
        if fail_count == 0 {
            GREEN_OK
        } else {
            RED_FAILED
        },
        pass_count,
        fail_count
    );

    Ok(fail_count)
}

fn run_one_test(test_case: &TestCase) -> bool {
    let test_name = test_case.name();
    if test_case.should_panic() {
        log::info!("test {} ... {}", test_name, GREEN_SKIP);
        true
    } else {
        // TBD: how to catch_unwind(|| test_case.func()()).is_err();
        test_case.func()();
        log::info!("test {} ... {}", test_name, GREEN_OK);
        true
    }
}

impl TestCase {
    pub fn new(name: &'static str, func: fn() -> (), should_panic: bool) -> Self {
        Self {
            name,
            func,
            should_panic,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn func(&self) -> fn() -> () {
        self.func
    }

    pub fn should_panic(&self) -> bool {
        self.should_panic
    }
}
