use router::Router;
use std::future::Future;
use std::net::{TcpListener, ToSocketAddrs};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use tokio::{runtime::Runtime, sync::Semaphore};
use types::{HttpRequest, HttpResponse};

struct SharedState {
    response: Option<HttpResponse>,
    waker: Option<Waker>,
}

pub struct RequestHandler {
    shared_state: Arc<Mutex<SharedState>>,
}

impl RequestHandler {
    pub fn new(request: HttpRequest) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            response: None,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            let resp = Router::dispatch(request);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.response = Some(resp);
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });
        RequestHandler { shared_state }
    }
}

impl Future for RequestHandler {
    type Output = HttpResponse;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        match &shared_state.response {
            Some(resp) => Poll::Ready(resp.clone()),
            _ => {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

struct Server<A>
where
    A: ToSocketAddrs,
{
    addr: A,
    sem: Arc<Semaphore>,
}

impl<A> Server<A>
where
    A: ToSocketAddrs + Copy,
{
    fn new(addr: A, max: usize) -> Self {
        let sem = Arc::new(Semaphore::new(max));
        Server {
            addr: addr,
            sem: sem,
        }
    }

    fn serve_forever(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // TODO stream to request
                    let s = Arc::clone(&self.sem);
                    let join_handle = thread::spawn(move || {
                        let rt = Runtime::new().unwrap();
                        rt.block_on(async {
                            let permit = s.acquire().await;
                            assert!(permit.is_err());
                            RequestHandler::new(HttpRequest {}).await
                        });
                    });
                }
                Err(e) => {
                    println!("return err response");
                }
            }
        }
        Ok(())
    }
}
