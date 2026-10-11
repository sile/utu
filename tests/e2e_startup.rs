//! End-to-end smoke test: the real `utu` binary starts, paints, and quits.
//!
//! This is the smallest test that proves the harness works at all. If the
//! binary cannot take over a PTY, or the emulator cannot read its output back,
//! this test fails and every other test in the suite is meaningless.

mod e2e;

use e2e::scratch_file;

#[test]
fn utu_starts_paints_and_quits() {
    let path = scratch_file("smoke.txt");
    std::fs::write(&path, "a.s\n").expect("write scratch file");

    let mut utu = e2e::UtuHarness::open(&path);

    // `open` waits for the first frame, so the status line is up.
    assert!(
        utu.screen_contains("[CANVAS:"),
        "expected the status line on screen:\n{}",
        utu.screen_text()
    );

    let status = utu.quit();
    assert!(status.success(), "utu exited with {status:?}");
}

#[test]
fn typing_a_dot_character_changes_the_canvas() {
    let path = scratch_file("draw.txt");
    std::fs::write(&path, "  \n  \n").expect("write scratch file");

    let mut utu = e2e::UtuHarness::open(&path);

    // `a` is bound to `dot(.)`, so it writes a `.` at the cursor.
    utu.send_char('a');
    utu.wait_for_text(".");

    let status = utu.quit();
    assert!(status.success(), "utu exited with {status:?}");
}
