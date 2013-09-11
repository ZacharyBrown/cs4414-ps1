//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::{str, io, path};

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut VISITOR_COUNT: int = 0;

fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
			unsafe {VISITOR_COUNT += 1;}
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        println(fmt!("Request received:\n%s", request_str));
			
			let filename = request_str.word_iter().skip(1).next().unwrap();
			
			//println(filename.slice_from(1));

			match filename {
			  "/" => {
			    let html_response: ~str = ~
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>
                             </body></html>\r\n";
			     let visitor_response: ~str = unsafe {fmt!("Number of requests: %i", VISITOR_COUNT)};
			     let response = html_response + visitor_response;
                             net_tcp::write(&sock, response.as_bytes_with_null_consume());
			     },
			  s => {
				let read_result = io::read_whole_file(~path::Path(s.slice_from(1)));
				let mut resp: ~[u8] = ~['.'as u8];
				match read_result {
				  
				  Ok(file) => {resp = file;},
				  Err(e) => {println(fmt!("Error reading file: %?", e));}
				};
			     net_tcp::write(&sock, resp);
			  }
			};
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
