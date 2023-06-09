fn main() {
    println!("Hello Bin!");
}

/// Requires running `cargo build` before running this test.
#[test]
fn test_cli() {
    use insta_cmd::{assert_cmd_snapshot, get_cargo_bin, Command};
    assert_cmd_snapshot!(Command::new(get_cargo_bin("hello")), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Hello Bin!

    ----- stderr -----
    "###);
}
