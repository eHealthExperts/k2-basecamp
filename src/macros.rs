macro_rules! test_server {
    (
        $(
            ($request:ident : &Request)
            $body: block
        )+
    ) => {
        $(
            {
                #[allow(unused)]
                let server = ::rouille::Server::new("127.0.0.1:0", move | $request | {
                    $body
                }).unwrap();

                ::std::env::set_var(
                    "K2_BASE_URL",
                    format!("{}{}", "http://", &server.server_addr()
                ));

                let (tx, rx) = ::std::sync::mpsc::channel();

                ::std::thread::spawn(move || loop {
                    server.poll();
                    match rx.try_recv() {
                        Ok(_) | Err(::std::sync::mpsc::TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(::std::sync::mpsc::TryRecvError::Empty) => {}
                    }
                });

                tx
            }
        )+
    }
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
