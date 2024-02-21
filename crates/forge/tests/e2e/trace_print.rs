use crate::e2e::common::runner::{setup_package, test_runner};
use indoc::indoc;
use test_utils::output_assert::assert_stdout_contains;

#[test]
fn trace_info_print() {
    let temp = setup_package("trace");
    let snapbox = test_runner();

    let output = snapbox.current_dir(&temp).assert().success();

    assert_stdout_contains(
        output,
        indoc! {r"
        [..]Compiling[..]
        [..]Finished[..]

        Collected 1 test(s) from trace_info package
        Running 0 test(s) from src/
        Running 1 test(s) from tests/
        Entry point type: External
        Selector: [..]
        Calldata: []
        Storage address: [..]
        Caller address: 0
        Call type: Call
        Nested Calls: [
            (
                Entry point type: External
                Selector: [..]
                Calldata: [..]
                Storage address: [..]
                Caller address: [..]
                Call type: Call
                Nested Calls: [
                    (
                        Entry point type: External
                        Selector: [..]
                        Calldata: [..]
                        Storage address: [..]
                        Caller address: [..]
                        Call type: Call
                        Nested Calls: [
                            (
                                Entry point type: External
                                Selector: [..]
                                Calldata: [0]
                                Storage address: [..]
                                Caller address: [..]
                                Call type: Call
                                Nested Calls: []
                            ),
                            (
                                Entry point type: External
                                Selector: [..]
                                Calldata: [0]
                                Storage address: [..]
                                Caller address: [..]
                                Call type: Call
                                Nested Calls: []
                            )
                        ]
                    ),
                    (
                        Entry point type: External
                        Selector: [..]
                        Calldata: [0]
                        Storage address: [..]
                        Caller address: [..]
                        Call type: Call
                        Nested Calls: []
                    )
                ]
            )
        ]
        
        [PASS] tests::test_trace::test_trace_print (gas: [..]
        Tests: 1 passed, 0 failed, 0 skipped, 0 ignored, 0 filtered out
        "},
    );
}