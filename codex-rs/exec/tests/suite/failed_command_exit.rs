#![cfg(not(target_os = "windows"))]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use core_test_support::responses;
use core_test_support::test_codex_exec::test_codex_exec;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exits_non_zero_when_shell_command_finishes_failed() -> anyhow::Result<()> {
    let test = test_codex_exec();

    let server = responses::start_mock_server().await;
    let response_streams = vec![
        responses::sse(vec![
            responses::ev_response_created("resp-1"),
            responses::ev_shell_command_call("call-1", "false"),
            responses::ev_completed("resp-1"),
        ]),
        responses::sse(vec![
            responses::ev_response_created("resp-2"),
            responses::ev_assistant_message("resp-2", "Ran `false`. It exited with status `1`."),
            responses::ev_completed("resp-2"),
        ]),
    ];
    let request_log = responses::mount_sse_sequence(&server, response_streams).await;

    test.cmd_with_server(&server)
        .arg("--ephemeral")
        .arg("--skip-git-repo-check")
        .arg("Run exactly: false and stop.")
        .assert()
        .code(1);

    let output = request_log
        .last_request()
        .expect("shell output request recorded")
        .function_call_output("call-1");
    assert_eq!(output["call_id"], "call-1");
    assert_eq!(output["type"], "function_call_output");
    assert!(output["output"].is_string());

    Ok(())
}
