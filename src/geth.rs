use std::{
    io::{Read, Write},
    process::{Child, Command, Stdio},
};

use crate::errors::{CommunicationError, CommunicationResult};

pub(crate) struct Geth(Child);

impl Default for Geth {
    fn default() -> Self {
        Geth(
            Command::new("./call_geth/call_geth")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("!child"),
        )
    }
}

impl Geth {
    pub fn run_precompile(&mut self, address: u8, input: &[u8]) -> CommunicationResult<Vec<u8>> {
        let stdin = self.0.stdin.as_mut().expect("!stdin");
        write_precompile_call(stdin, address, input)?;

        let stdout = self.0.stdout.as_mut().expect("!stdout");
        read_precompile_result(stdout)
    }
}

fn write_precompile_call<W>(w: &mut W, address: u8, buf: &[u8]) -> CommunicationResult<()>
where
    W: Write,
{
    w.write_all(&(buf.len() as u16).to_be_bytes()[..])?;
    w.write_all(&[address])?;
    w.write_all(buf)?;
    Ok(())
}

fn read_precompile_result<R>(r: &mut R) -> CommunicationResult<Vec<u8>>
where
    R: Read,
{
    let mut body_size = [0u8; 2];
    r.read_exact(&mut body_size)?;
    let body_size = u16::from_be_bytes(body_size) as usize;

    let mut is_err = [0u8];
    r.read_exact(&mut is_err)?;

    let mut body = vec![0u8; body_size];
    r.read_exact(&mut body[..body_size])?;

    if is_err[0] == 1 {
        Err(CommunicationError::RemoteError(
            String::from_utf8(body).unwrap(),
        ))
    } else {
        Ok(body)
    }
}
