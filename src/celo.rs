use lazy_static::lazy_static;

use std::{
    io::{Read, Result as IoResult, Write},
    process::{Child, Command, Stdio},
    sync::Mutex,
};

lazy_static! {
    static ref CELO: Mutex<Child> = Mutex::new(
        Command::new("./call_celo/call_celo")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("!child")
    );
}

fn write_precompile_call<W>(w: &mut W, address: u8, buf: &[u8]) -> IoResult<()>
where
    W: Write,
{
    w.write_all(&(buf.len() as u16).to_be_bytes()[..])?;
    w.write_all(&[address])?;
    w.write_all(buf)?;
    Ok(())
}

fn read_precompile_result<R>(r: &mut R) -> Result<Vec<u8>, String>
where
    R: Read,
{
    let mut body_size = [0u8; 2];
    r.read_exact(&mut body_size).unwrap();
    let body_size = u16::from_be_bytes(body_size) as usize;

    let mut is_err = [0u8];
    r.read_exact(&mut is_err).unwrap();

    let mut body = vec![0u8; body_size];
    r.read_exact(&mut body[..body_size]).unwrap();

    if is_err[0] == 1 {
        Err(String::from_utf8(body).unwrap())
    } else {
        Ok(body)
    }
}

pub fn run_precompile(address: u8, input: &[u8]) -> Result<Vec<u8>, String> {
    // println!("{:?}", std::env::current_dir().unwrap());
    let mut c = CELO.lock().unwrap();

    let stdin = c.stdin.as_mut().expect("!stdin");

    write_precompile_call(stdin, address, input).unwrap();

    let stdout = c.stdout.as_mut().expect("!stdout");
    read_precompile_result(stdout)
}
