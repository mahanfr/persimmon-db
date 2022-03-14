use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};

#[path = "../test/files.rs"]
mod test;
#[path = "database/database.rs"]
mod db;

const SERVER_ADD : &str = "127.0.0.1:4485";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADD).await.unwrap();
    println!("Listening on address: {}",SERVER_ADD);

    let flags = 125;
    println!("{}", format!("{:032b}",flags));


    loop {
        let (stream,_) = listener.accept().await.unwrap();
        handel_connection(stream).await;
    }
}

async fn handel_connection(mut stream:TcpStream) {
    let mut buffer = [0;1024];
    let len = stream.read(&mut buffer).await.unwrap();
    // let message = String::from_utf8_lossy(&buffer[..len]);
    if len < 20 {
        println!("{}", "Invalid request");
        return;
    }else{
        let message = parse_request(&buffer[..len]).await;
        println!("{:?}", message);
    }
}


#[derive(Debug)]
enum OpCode {
    Reply = 1,
    Update = 2001,
    Insert = 2002,
    Query = 2004,
    GetMore= 2005,
    Delete = 2006,
    KillCursors = 2007,
    Msg = 2013,
}

#[derive(Debug)]
struct Message {
    len: u32,
    request_id: u32,
    response_to: u32,
    op_code: OpCode,
    checksum_present: bool,
    more_to_come: bool,
    exhaust_allowed: bool,
    value: String,
    checksum: u32,
}

fn parse_flags(flags:u32) -> (bool,bool,bool) {
    let mut checksum_present = false;
    let mut more_to_come = false;
    let mut exhaust_allowed = false;
    let flag_to_bits = format!("{:032b}",flags);
    if flag_to_bits.len() > 1 {
        checksum_present = flag_to_bits.chars().nth(0).unwrap() == '1';
        more_to_come = flag_to_bits.chars().nth(1).unwrap() == '1';
        exhaust_allowed = flag_to_bits.chars().nth(16).unwrap() == '1';
    }
    (checksum_present,more_to_come,exhaust_allowed)
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

// TODO: add support for all Opcodes
async fn parse_request(buffer:&[u8]) -> Box<Message> {
    let len = as_u32_be(&[buffer[0],buffer[1],buffer[2],buffer[3]]);
    let request_id = as_u32_be(&[buffer[4],buffer[5],buffer[6],buffer[7]]);
    let response_to = as_u32_be(&[buffer[8],buffer[9],buffer[10],buffer[11]]);
    let op_code_int = as_u32_be(&[buffer[12],buffer[13],buffer[14],buffer[15]]);
    let op_code: OpCode = match op_code_int {
        1 => OpCode::Reply,
        2001 => OpCode::Update,
        2002 => OpCode::Insert,
        2004 => OpCode::Query,
        2005 => OpCode::GetMore,
        2006 => OpCode::Delete,
        2007 => OpCode::KillCursors,
        2013 => OpCode::Msg,
        _ => panic!("Unknown op code: {}",op_code_int),
    };
    let (checksum_present,more_to_come,exhaust_allowed) = 
        parse_flags(as_u32_be(&[buffer[16],buffer[17],buffer[18],buffer[19]]));
    let value_len ;
    if checksum_present {
        value_len = len - 4;
    } else {
        value_len = len;
    }
    let value = String::from_utf8_lossy(&buffer[20..(value_len) as usize]).to_string();
    let checksum = if checksum_present {
        as_u32_be(&[buffer[(value_len) as usize],buffer[(value_len+1) as usize],buffer[(value_len+2) as usize],buffer[(value_len+3) as usize]])
    } else {
        0
    };
    Box::new(Message {
        len,
        request_id,
        response_to,
        op_code,
        checksum_present,
        more_to_come,
        exhaust_allowed,
        value,
        checksum,
    })
}

