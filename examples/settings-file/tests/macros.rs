#[macro_export]
macro_rules! test_server {
    (
        $(
            ($addr:expr, $request:ident : &Request)
            $body: block
        )+
    ) => {
        $(
            {
                #[allow(unused)]
                let server = ::rouille::Server::new($addr, move | $request | {
                    $body
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
            }
        )+
    }
}
