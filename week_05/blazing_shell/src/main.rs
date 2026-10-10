use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("enter command ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => {}
            Err(e) => {
                println!("error reading input: {}", e);
                continue;
            }
        }

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line == "exit" {
            break;
        }

        if let Err(e) = execute(line) {
            println!("error: {}", e);
        }
    }
}

fn execute(line: &str) -> Result<(), String> {
    let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();

    match parts.as_slice() {
        [single] => run_single(single),
        [left, right] => run_piped(left, right),
        _ => Err("support only 2 commands, splitted by '|'".to_string()),
    }
}

fn parse_command(s: &str) -> Result<(String, Vec<String>), String> {
    let mut tokens = s.split_whitespace();
    let cmd = tokens
        .next()
        .ok_or_else(|| "empty command".to_string())?
        .to_string();
    let args: Vec<String> = tokens.map(String::from).collect();
    Ok((cmd, args))
}

fn run_single(s: &str) -> Result<(), String> {
    let (cmd, args) = parse_command(s)?;

    let args_string = args.join(" ");

    let command = format!("{} {}", cmd, args_string);

    // Да у меня винда)
    let status = Command::new("cmd")
        .args(["/C", &command])
        .stdout(Stdio::inherit())
        .status();

    match status {
        Ok(st) => {
            if !st.success() {
                println!("{:?}", st);
            }
            Ok(())
        }
        Err(e) => Err(format!("failed to execute '{}': {}", &command, e)),
    }
}

fn run_piped(left: &str, right: &str) -> Result<(), String> {
    let (cmd1, args1) = parse_command(left)?;
    let (cmd2, args2) = parse_command(right)?;

    let args_string1 = args1.join(" ");
    let args_string2 = args2.join(" ");

    let command1 = format!("{} {}", cmd1, args_string1);
    let command2 = format!("{} {}", cmd2, args_string2);

    let mut child_left = Command::new("cmd")
        .args(["/C", &command1])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to execute '{}': {}", command1, e))?;

    let child_left_stdout = child_left
        .stdout
        .take()
        .ok_or_else(|| "не удалось получить stdout первой команды".to_string())?;

    let mut child_right = Command::new("cmd")
        .args(["/C", &command2])
        .stdin(Stdio::from(child_left_stdout))
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|e| {
            // не забываем про первый запущенный подпроцесс
            let _ = child_left.kill();
            let _ = child_left.wait();
            format!("failed to execute '{}': {}", command2, e)
        })?;

    let status_right = child_left
        .wait()
        .map_err(|e| format!("error for '{}': {}", command1, e))?;
    let status_left = child_right
        .wait()
        .map_err(|e| format!("error for '{}': {}", command2, e))?;

    if !status_right.success() {
        println!(
            "process '{}' exited with status: {}",
            command1, status_right
        );
    }
    if !status_left.success() {
        println!("process '{}' exited with status: {}", command2, status_left);
    }

    Ok(())
}
