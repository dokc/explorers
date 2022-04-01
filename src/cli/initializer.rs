use colored::Colorize;
use derive;
use std::io;

#[derive(Debug)]
pub struct ProtocolOptions {
    _port: u32,
}
#[derive(Debug)]
pub struct QueryOptions {
    _consistency: String,
}

#[derive(Debug)]
pub struct SocketOptions {
    _read_timeout: u64,
}

#[derive(Debug)]
pub struct ExpressConnection {
    _size_of_contact_points: u8,
    _contact_points: Vec<String>,
    _localdatacenter: String,
    _keyspace: String,
    _protocol_options: ProtocolOptions,
    _socket_options: SocketOptions,
    _query_options: QueryOptions,
}

pub fn constructor(
    size: u8,
    contact: Vec<String>,
    localdc: String,
    keyspace: String,
    socketopt: SocketOptions,
    protoc_options: ProtocolOptions,
    queryopt: QueryOptions,
) -> ExpressConnection {
    ExpressConnection {
        _size_of_contact_points: size,
        _contact_points: contact,
        _localdatacenter: localdc,
        _keyspace: keyspace,
        _socket_options: socketopt,
        _protocol_options: protoc_options,
        _query_options: queryopt,
    }
}

pub fn construct_express() -> ExpressConnection {
    let mut _size: String = String::new();
    println!(
        "{}",
        format!("{}", "Enter the size of contact points").green()
    );
    io::stdin()
        .read_line(&mut _size)
        .expect("Failed to read line");
    let size: u8 = _size.trim().parse().unwrap();

    let mut contact: Vec<String> = vec![];
    for _i in 0..(size) {
        let mut inst: String = String::new();
        println!(
            "{}",
            format!("{}", "Enter the contact points sequentially").yellow()
        );

        io::stdin()
            .read_line(&mut inst)
            .expect("Failed to read line");
        let data = inst.trim();
        contact.push(data.to_string());
    }

    let mut localdc: String = String::new();
    println!(
        "{}",
        format!("{}", "Enter the name of your local data center.").yellow()
    );
    io::stdin()
        .read_line(&mut localdc)
        .expect("Failed to read line");
    let mut keyspace: String = String::new();
    println!(
        "{}",
        format!("{}", "Enter the name of your keyspace").yellow()
    );
    io::stdin()
        .read_line(&mut keyspace)
        .expect("Failed to read line");

    let mut _ports: String = String::new();
    println!("{}", format!("{}", "Enter the port number").yellow());
    io::stdin().read_line(&mut _ports).expect("Failed to read");
    let prot_instance: ProtocolOptions = ProtocolOptions {
        _port: _ports.trim().parse().unwrap(),
    };
    println!(
        "{}",
        format!("{}", "Enter the name of your consistency").yellow()
    );
    let mut query_inst: String = String::new();
    io::stdin()
        .read_line(&mut query_inst)
        .expect("Failed to read");
    let queryopt_inst: QueryOptions = QueryOptions {
        _consistency: query_inst.trim().to_string(),
    };
    println!(
        "{}",
        format!("{}", "Enter the time in seconds for read timeout.").yellow()
    );
    let mut sockt_inst: String = String::new();
    io::stdin()
        .read_line(&mut sockt_inst)
        .expect("Failed to read");
    let socktoptions_inst: SocketOptions = SocketOptions {
        _read_timeout: sockt_inst.trim().parse().unwrap(),
    };
    let query_recognized: ExpressConnection = constructor(
        size,
        contact,
        localdc.trim().to_string(),
        keyspace.trim().to_string(),
        socktoptions_inst,
        prot_instance,
        queryopt_inst,
    );
    return query_recognized;
}

pub fn create() {
    println!("This configurator will configure your Express-Cassandra database connection\n");
    let message: String = String::from(
        "Welcome to the constructor, the constructor will construct the client for you.\nThe client will be structured according to the example",

    );
    println!("{}", format!("{}", message).green());

    println!("\nCheck the example down below\n");

    let instance: ExpressConnection = constructor(
        3,
        vec![String::from("127.0.7.1")],
        String::from("datacenter"),
        String::from("test_keyspace"),
        SocketOptions {
            _read_timeout: 6000,
        },
        ProtocolOptions { _port: 80 },
        QueryOptions {
            _consistency: String::from("ExpressCassandra.consistencies.one"),
        },
    );
    println!("{:#?}", instance);

    let message: String = String::from("You have called the config command, the CLI will now ask you sequentially to confirm your options.");
    println!("{}", format!("{}", message).blue().on_bright_white());

    let data: ExpressConnection = construct_express();

    println!("{}", format!("\n{:#?}", data).green());
    println!("Is this the configuration you'd like to have ?");
    println!(
        "{}, {}",
        format!("Type Y for yes").green(),
        format!("\nand N for no").red()
    );

    let mut cmd: String = String::new();
    io::stdin().read_line(&mut cmd).expect("Failed to read");

    while cmd.trim() == "N" {
        let data: ExpressConnection = construct_express();
        println!("{}", format!("\n{:#?}", data).green());
        println!(
            "{}, {}",
            format!("Type Y for yes").green(),
            format!("\nand N for no").red()
        );
        io::stdin().read_line(&mut cmd).expect("Failed to read");
    }
}
