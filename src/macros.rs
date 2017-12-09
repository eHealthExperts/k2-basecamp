macro_rules! test_server {
    ({($method:ident) ($($url:tt)+) => $handle:block}) => ({
        let addr = String::from("127.0.0.1:54321");
        ::std::env::set_var("K2_BASE_URL", format!("{}{}", "http://", &addr));

        let server = ::rouille::Server::new(addr.clone(), move |request| {
            router!(request,
                ($method) ($($url)+) => $handle,
                _ => {
                    assert!(false);
                    ::rouille::Response::empty_404()
                }
            )
        }).unwrap();

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
    })
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
