fn main() {
    println!("Hello Example!");
}

#[test]
fn test_cli() {
    use insta_cmd::{assert_cmd_snapshot, get_cargo_example, Command};
    assert_cmd_snapshot!(Command::new(get_cargo_example("hello")), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Hello Example!

    ----- stderr -----
    "###);
}
