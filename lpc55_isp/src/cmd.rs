use crate::isp::*;
use anyhow::Result;

enum DataProc<'a> {
    NoData,
    SendData {
        code: ResponseCode,
        data: &'a Vec<u8>,
    },
    RecvData {
        code: ResponseCode,
        cnt: u32,
    },
}

fn do_command(
    port: &mut dyn serialport::SerialPort,
    tag: CommandTag,
    command_resp: ResponseCode,
    args: Vec<u32>,
    d: DataProc,
) -> Result<Option<Vec<u8>>> {
    send_command(port, tag, args)?;

    read_response(port, command_resp)?;

    let ret = match d {
        DataProc::NoData => None,
        DataProc::SendData { code, data } => {
            send_data(port, data)?;
            read_response(port, code)?;
            None
        }
        DataProc::RecvData { code, cnt } => {
            let r = recv_data(port, cnt)?;
            read_response(port, code)?;
            Some(r)
        }
    };

    Ok(ret)
}

pub fn do_save_keystore(port: &mut dyn serialport::SerialPort) -> Result<()> {
    let mut args: Vec<u32> = Vec::new();

    // Arg 0 =  WriteNonVolatile
    args.push(KeyProvisionCmds::WriteNonVolatile as u32);
    // Arg 1 = Memory ID (0 = internal flash)
    args.push(0);

    do_command(
        port,
        CommandTag::KeyProvision,
        ResponseCode::GenericResponse,
        args,
        DataProc::NoData,
    )?;

    Ok(())
}

pub fn do_enroll(port: &mut dyn serialport::SerialPort) -> Result<()> {
    let mut args: Vec<u32> = Vec::new();

    // Arg =  Enroll
    args.push(KeyProvisionCmds::Enroll as u32);

    do_command(
        port,
        CommandTag::KeyProvision,
        ResponseCode::GenericResponse,
        args,
        DataProc::NoData,
    )?;

    Ok(())
}

pub fn do_generate_uds(port: &mut dyn serialport::SerialPort) -> Result<()> {
    let mut args: Vec<u32> = Vec::new();

    // Arg 0 =  SetIntrinsicKey
    args.push(KeyProvisionCmds::SetIntrinsicKey as u32);
    // Arg 1 = UDS
    args.push(KeyType::UDS as u32);
    // Arg 2 = size
    args.push(32);

    do_command(
        port,
        CommandTag::KeyProvision,
        ResponseCode::GenericResponse,
        args,
        DataProc::NoData,
    )?;

    Ok(())
}

pub fn do_isp_write_keystore(port: &mut dyn serialport::SerialPort, data: Vec<u8>) -> Result<()> {
    let mut args = Vec::new();

    args.push(KeyProvisionCmds::WriteKeyStore as u32);

    do_command(
        port,
        CommandTag::KeyProvision,
        ResponseCode::KeyProvisionResponse,
        args,
        DataProc::SendData {
            code: ResponseCode::GenericResponse,
            data: &data,
        },
    )?;

    Ok(())
}

pub fn do_recv_sb_file(port: &mut dyn serialport::SerialPort, data: Vec<u8>) -> Result<()> {
    let mut args = Vec::new();

    // Arg0 = File len
    args.push(data.len() as u32);

    do_command(
        port,
        CommandTag::ReceiveSbFile,
        ResponseCode::GenericResponse,
        args,
        DataProc::SendData {
            code: ResponseCode::GenericResponse,
            data: &data,
        },
    )?;

    Ok(())
}

pub fn do_isp_set_userkey(
    port: &mut dyn serialport::SerialPort,
    key_type: KeyType,
    data: Vec<u8>,
) -> Result<()> {
    let mut args = Vec::new();

    // Arg0 = Set User Key
    args.push(KeyProvisionCmds::SetUserKey as u32);
    // Arg1 =  Key type
    args.push(key_type as u32);
    // Arg2 = Key size
    args.push(data.len() as u32);

    do_command(
        port,
        CommandTag::KeyProvision,
        ResponseCode::KeyProvisionResponse,
        args,
        DataProc::SendData {
            code: ResponseCode::GenericResponse,
            data: &data,
        },
    )?;

    Ok(())
}

pub fn do_isp_read_memory(
    port: &mut dyn serialport::SerialPort,
    address: u32,
    cnt: u32,
) -> Result<Vec<u8>> {
    let mut args = Vec::new();

    args.push(address);
    args.push(cnt);
    args.push(0x0);

    let f = do_command(
        port,
        CommandTag::ReadMemory,
        ResponseCode::ReadMemoryResponse,
        args,
        DataProc::RecvData {
            code: ResponseCode::GenericResponse,
            cnt,
        },
    )?;

    Ok(f.unwrap())
}

pub fn do_isp_write_memory(
    port: &mut dyn serialport::SerialPort,
    address: u32,
    data: Vec<u8>,
) -> Result<()> {
    let mut args = Vec::new();

    args.push(address);
    args.push(data.len() as u32);
    args.push(0x0);

    do_command(
        port,
        CommandTag::WriteMemory,
        ResponseCode::GenericResponse,
        args,
        DataProc::SendData {
            code: ResponseCode::GenericResponse,
            data: &data,
        },
    )?;

    Ok(())
}

pub fn do_isp_flash_erase_all(port: &mut dyn serialport::SerialPort) -> Result<()> {
    let mut args: Vec<u32> = Vec::new();

    // Erase internal flash
    args.push(0x0 as u32);

    do_command(
        port,
        CommandTag::FlashEraseAll,
        ResponseCode::GenericResponse,
        args,
        DataProc::NoData,
    )?;

    Ok(())
}
