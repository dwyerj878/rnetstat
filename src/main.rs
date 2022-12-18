use netstat::{iterate_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpSocketInfo, UdpSocketInfo};
use clap::Parser;

#[derive(Parser, Default, Debug)]
struct Cli {
    // ip v4 or v6 both if unspecified
    #[arg(short = '4', name = "4", required = false,  help = "show ip 4 only", conflicts_with("6"))]
    ip4: bool,

    #[arg(short = '6', name = "6", required = false,  help = "show ip 6 only", conflicts_with("4"))]
    ip6: bool,

    #[arg(short = 't', name = "T", required = false,  help = "show tcp only", conflicts_with("U"))]
    tcp: bool,

    #[arg(short = 'u', name = "U", required = false,  help = "show udp only", conflicts_with("T"))]
    udp: bool
}

fn main() {
    println!("Start !");
    let cli = Cli::parse();
    println!("{:?}", cli);

    let af_flags = extract_family(&cli);    
    let proto_flags = extract_protocol(&cli);

    let sockets = iterate_sockets_info(af_flags, proto_flags);
    if sockets.is_err() {
        println!("Error !!");
    } else {
        println!("OK!");
        for x in sockets.ok() {
            for y in x {
                if y.is_err() {
                    println!("error");
                } else {                    
                    let s = y.ok();
                    let info = s.unwrap();
                    let psi = info.protocol_socket_info;
                    
                    match psi {
                        ProtocolSocketInfo::Tcp(p) => show_tcp(p),
                        ProtocolSocketInfo::Udp(p) => show_udp(p)
                    }
                }
            }
        }
    }
    println!("... Done");

}

fn extract_protocol(cli: &Cli)  -> ProtocolFlags {
    let proto_flags: ProtocolFlags = if cli.tcp {
        ProtocolFlags::TCP
    } else if cli.udp {
        ProtocolFlags::UDP
    } else {
        ProtocolFlags::all()
    };
    proto_flags
}

fn extract_family(cli: &Cli) -> AddressFamilyFlags {
    let af_flags: AddressFamilyFlags = if cli.ip4 {
        AddressFamilyFlags::IPV4
    } else if cli.ip6 {
        AddressFamilyFlags::IPV6
    } else {
        AddressFamilyFlags::all()
    };
    af_flags
}

fn show_udp(p : UdpSocketInfo) {
    println!("udp   {:>39}:{:>5} ",p.local_addr, p.local_port  );
}

fn show_tcp(p : TcpSocketInfo) {
    
    //tcp        0      0 10.0.0.167:57890        51.81.245.207:443       ESTABLISHED
    println!("tcp    {:>39}:{:>5} => {:>39}:{:>5} {}",  p.local_addr,p.local_port,  p.remote_addr, p.remote_port,p.state );
/*    match p.state {
        netstat::TcpState::Closed => todo!(),
        netstat::TcpState::Listen => todo!(),
        netstat::TcpState::SynSent => todo!(),
        netstat::TcpState::SynReceived => todo!(),
        netstat::TcpState::Established => todo!(),
        netstat::TcpState::FinWait1 => todo!(),
        netstat::TcpState::FinWait2 => todo!(),
        netstat::TcpState::CloseWait => todo!(),
        netstat::TcpState::Closing => todo!(),
        netstat::TcpState::LastAck => todo!(),
        netstat::TcpState::TimeWait => todo!(),
        netstat::TcpState::DeleteTcb => todo!(),
    } */
}