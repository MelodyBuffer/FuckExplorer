use std::process::Command;

fn main() {
    let script = r#"
        [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
        taskkill /f /im explorer.exe
        Start-Process explorer.exe
    "#;

    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(script)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", stderr);
    }
}
