use std::process::Command;

fn main() {
    println!("Hello, world!");
    let output = Command::new("powershell")
        .args(["/C", r#"(get-item ($env:SystemDrive + "\Program Files (x86)\Microsoft\Edge\Application\msedge.exe")).VersionInfo.FileVersion"#])
        .output()
        .expect("failed to execute process");
    let edge_version = output.stdout;
    println!("{}", std::str::from_utf8(&edge_version).unwrap());
}
