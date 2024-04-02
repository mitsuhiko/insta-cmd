#[macro_export]
#[doc(hidden)]
macro_rules! _run_and_snapshot {
    ($cmd:expr, $body:expr) => {{
        let (info, output) = $crate::_macro_support::Spawn::spawn_with_info(&mut $cmd, None);
        let mut settings = $crate::_macro_support::insta::Settings::clone_current();
        settings.set_info(&info);
        settings.set_omit_expression(true);
        settings.bind(|| {
            #[allow(clippy::redundant_closure_call)]
            ($body)(&format!(
                "success: {:?}\nexit_code: {}\n----- stdout -----\n{}\n----- stderr -----\n{}",
                output.status.success(),
                output.status.code().unwrap_or(!0),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        });
    }};
}

/// Runs an [spawnable](crate::Spawn) and snapshots the output.
#[macro_export]
macro_rules! assert_cmd_snapshot {
    ($spawnable:expr, @$snapshot:literal $(,)?) => {{
        #[allow(unused)]
        use $crate::SpawnExt;
        $crate::_run_and_snapshot!($spawnable, |snapshot: &str| {
            $crate::_macro_support::insta::assert_snapshot!(snapshot, @$snapshot);
        });
    }};
    ($name:expr, $spawnable:expr $(,)?) => {{
        #[allow(unused)]
        use $crate::SpawnExt;
        $crate::_run_and_snapshot!($spawnable, |snapshot: &str| {
            $crate::_macro_support::insta::assert_snapshot!($name, snapshot);
        });
    }};
    ($spawnable:expr $(,)?) => {{
        #[allow(unused)]
        use $crate::SpawnExt;
        $crate::_run_and_snapshot!($spawnable, |snapshot: &str| {
            $crate::_macro_support::insta::assert_snapshot!(snapshot);
        });
    }};
}
