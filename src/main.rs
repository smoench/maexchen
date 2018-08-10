use std::net::UdpSocket;

fn main() {
    println!("Hello World!");

    let receiver = "192.168.179.27:9000";
    let socket = connect("0.0.0.0:3400");

    send(&socket, receiver, "REGISTER;Simon");

    let mut round_started = false;
    let mut my_dice: String = String::new();
    let mut last_announced_dice: String = String::new();

    loop {
        let msg_bytes = listen(&socket);
        let msg = String::from_utf8(msg_bytes).unwrap();

        if msg.len() == 0 {
            continue;
        }

        let split_msg: Vec<&str> = msg.split(';').collect();

        if split_msg.len() == 0 {
            continue;
        }

        if split_msg[0] == "ROUND STARTING" {
            round_started = true;
            my_dice = String::new();

            send(&socket, receiver, format!("JOIN;{}", split_msg[1]).as_str());
        }

        if round_started == false {
            continue;
        }

        if split_msg[0] == "ROUND CANCELLED" {
            round_started = false;
        }

        if split_msg[0] == "YOUR TURN" {
            if my_dice.len() == 0 {
                send(&socket, receiver, format!("ROLL;{}", split_msg[1]).as_str());
            } else {
                send(&socket, receiver, format!("SEE;{}", split_msg[1]).as_str());
            }
        }

        if split_msg[0] == "ROLLED" {
            my_dice = split_msg[1].to_string();

            send(&socket, receiver, format!(
                "ANNOUNCE;{};{}",
                split_msg[1],
                split_msg[2]
            ).as_str());
        }

        if split_msg[0] == "ANNOUNCED" {
            last_announced_dice = split_msg[2].to_string();
        }
    }
}

fn connect(host: &str) -> UdpSocket {
    let socket = UdpSocket::bind(host).expect("couldn't bind to address");

    let duration = std::time::Duration::new(5, 0);
    let dur = std::option::Option::Some(duration);
    let _res = socket.set_read_timeout(dur).expect("failed to set timeout");

    socket
}

fn listen(socket: &UdpSocket) -> Vec<u8> {
    let mut buf = [0; 2048];
    let mut result: Vec<u8> = Vec::new();

    match socket.recv_from(&mut buf) {
        Ok((number_of_bytes, _src_addr)) => {
            result = Vec::from(&buf[0..number_of_bytes]);
        }
        Err(fail) => println!("failed listening {:?}", fail)
    }

    let display_result = result.clone();
    let result_str = String::from_utf8(display_result).unwrap();
    println!("received message: {:?}", result_str);

    result
}

fn send(socket: &UdpSocket, receiver: &str, msg: &str) -> usize {
    let string_msg = String::from(msg);
    let string_msg_bytes = string_msg.into_bytes();

    println!("sending message: {:?}", msg);

    let result: usize = 0;
    match socket.send_to(&string_msg_bytes, receiver) {
        Ok(_number_of_bytes) => {}
        Err(fail) => println!("failed sending {:?}", fail),
    }

    result
}

fn is_greater(a: String, b: String) -> bool {
    let hash: Vec<&str> = vec![
        "3,1",
        "3,2",
        "4,1",
        "4,2",
        "4,3",
        "5,1",
        "5,2",
        "5,3",
        "5,4",
        "6,1",
        "6,2",
        "6,3",
        "6,4",
        "6,5",
        "1,1",
        "2,2",
        "3,3",
        "4,4",
        "5,5",
        "6,6",
        "2,1",
    ];

    true
}