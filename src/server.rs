use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::Write;

use std::sync::Arc;

use request::Request;
use thread_pool::ThreadPool;
use router::{Route, Router, Handler};


pub struct AramidServer {
    pool: ThreadPool,
    router: Router,
}

impl AramidServer {
    pub fn new(num_workers: usize) -> AramidServer {
        AramidServer {
            pool: ThreadPool::new(num_workers),
            router: Router::new(),
        }
    }

    pub fn listen<A: ToSocketAddrs>(&self, addr: A) {
        let listener = TcpListener::bind(addr).unwrap();

        let router_arc = Arc::new(self.router.clone());

        for stream in listener.incoming() {
            let router = router_arc.clone();

            self.pool.execute(|| {
                handle_client(stream.unwrap(), router);

            });
            println!("executed");
        }
    }

    pub fn handle(&mut self, path: &str, handler: Handler) {
        self.router.add_route(Route::new(path, handler));
    }

}

fn handle_client(mut stream: TcpStream, router: Arc<Router>) {
    let mut request = Request::from_tcp_stream(&stream);
    let route = router.get_route(&request);
    let response_string = route.handle(&mut request).as_http_string();

    let _ = stream.write(response_string.as_bytes());
}
