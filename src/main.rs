extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use router::Router;

use std::sync::Mutex;

struct Counter {
	count: u64,
}

impl Counter {
	pub fn new() -> Counter {
		Counter {
			count: 0,
		}
	}

	pub fn tick(&mut self) {
		self.count += 1;
	}
}

struct CounterHandler {
	counter: Mutex<Counter>,
}

impl CounterHandler {
	pub fn new() -> CounterHandler {
		let fresh_counter = Mutex::new(Counter::new());

		CounterHandler {
			counter: fresh_counter,
		}
	}
}

impl Handler for CounterHandler {
	fn handle(&self, _: &mut Request) -> IronResult<Response> {

		let mut counter = self.counter.lock().unwrap();
	    let payload = format!("Hello from request number {}", &counter.count);
	    counter.tick();

	    Ok(Response::with((status::Ok, payload)))
	}
}

fn main() {
    let mut router = Router::new();
    let counter_handler = CounterHandler::new();

    router.get("/", counter_handler);

    Iron::new(router).http("localhost:8081").unwrap();
    println!("On 8081");
}
