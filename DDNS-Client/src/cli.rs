use crate::{
    parser,
    parser::{Args, Auth},
};
use anyhow::{Context, Result, bail};
use secrecy::{ExposeSecret, SecretString};
use std::{io, io::BufRead};

pub fn cli() -> Result<()> {
    let args = parser::parse_args();
    match args.cmd {
        parser::Cmd::Auth(auth) => match auth.cmd {
            parser::AuthCmd::Login(login) => run_login(login),
            parser::AuthCmd::Logout(_) => run_logout(),
        },
    }
}

fn run_login(login: parser::Login) -> Result<()> {
    // Implement login logic here
    let password = obtain_password(&login)?;
    let _pw = password.expose_secret();
    dbg!(_pw);
    Ok(())
}
fn run_logout() -> Result<()> {
    // Implement logout logic here
    Ok(())
}

fn obtain_password(login: &parser::Login) -> Result<SecretString> {
    let specified =
        login.password.is_some() as u8 + login.password_prompt as u8 + login.password_stdin as u8;
    let mut is_default = false;
    if specified == 0 {
        is_default = true;
    }
    if specified > 1 {
        bail!(
            "請僅選擇一種密碼來源（互斥）：--password、--password-prompt (-t)、或 \
             --password-stdin (-i)"
        );
    }
    if !is_default {
        if let Some(p) = &login.password {
            return Ok(SecretString::new(p.clone().into_boxed_str()));
        }

        if login.password_stdin {
            return read_password_from_stdin();
        }
    }
    let pw = cliclack::password("Password")
        .mask('•')
        .validate(|s: &String| if s.is_empty() { Err("密碼不可為空") } else { Ok(()) })
        .interact()
        .context("互動式輸入密碼失敗")?;
    Ok(SecretString::new(pw.into_boxed_str()))
}

fn read_password_from_stdin() -> Result<SecretString> {
    if atty::is(atty::Stream::Stdin) {
        bail!("偵測到是互動終端，請改用 -t 啟用遮罩輸入，或用 pipe 將密碼送入 -i");
    }
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    let _ = handle.read_line(&mut buf).context("讀取 stdin 發生錯誤")?;
    let trimmed = buf.trim_end_matches(['\r', '\n']).to_string();

    if trimmed.is_empty() {
        bail!("stdin 為空，請確認有透過管線輸入密碼");
    }

    Ok(SecretString::new(trimmed.into_boxed_str()))
}
