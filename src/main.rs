use netstat::{iterate_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpSocketInfo, UdpSocketInfo};

fn main() {
    println!("Hello, world!");
    let af_flags = AddressFamilyFlags::all();
    let proto_flags: ProtocolFlags = ProtocolFlags::all();

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

fn show_udp(p : UdpSocketInfo) {
    println!("UDP => lp: {} ", p.local_port );
}

fn show_tcp(p : TcpSocketInfo) {
    println!("TCP => {} ({}) => {} ({}) {}", p.local_port, p.local_addr, p.remote_port, p.remote_addr, p.state );
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