pub fn git_cmd() -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.stdout(std::process::Stdio::null());
    cmd.stderr(std::process::Stdio::null());
    cmd
}

