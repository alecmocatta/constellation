//= {
//=   "output": {
//=     "2": [
//=       "",
//=       true
//=     ],
//=     "1": [
//=       "\\[\"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\", \"abcdefghijklmno\"\\]\n",
//=       true
//=     ]
//=   },
//=   "children": [
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "hi 4\ndone 4\n",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "hi 6\ndone 6\n",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "1": [
//=           "hi 7\ndone 7\n",
//=           true
//=         ],
//=         "2": [
//=           "",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "hi 9\ndone 9\n",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "1": [
//=           "hi 1\ndone 1\n",
//=           true
//=         ],
//=         "2": [
//=           "",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "1": [
//=           "hi 5\ndone 5\n",
//=           true
//=         ],
//=         "2": [
//=           "",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "hi 3\ndone 3\n",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "hi 0\ndone 0\n",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "1": [
//=           "hi 2\ndone 2\n",
//=           true
//=         ],
//=         "2": [
//=           "",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     },
//=     {
//=       "output": {
//=         "2": [
//=           "",
//=           true
//=         ],
//=         "1": [
//=           "hi 8\ndone 8\n",
//=           true
//=         ]
//=       },
//=       "children": [],
//=       "exit": {
//=         "Left": 0
//=       }
//=     }
//=   ],
//=   "exit": {
//=     "Left": 0
//=   }
//= }

#![deny(warnings, deprecated)]
extern crate deploy;
extern crate futures;
use deploy::*;
use futures::{future::FutureExt, sink::SinkExt, stream::StreamExt};

fn sub(parent: Pid, arg: u32) {
	println!("hi {}", arg);
	let (receiver, sender) = (
		Receiver::<Option<String>>::new(parent),
		Sender::<Option<String>>::new(parent),
	);
	let () = futures::executor::block_on(
		receiver.forward(sender)
		.and_then(|(_stream,sink)|sink.close()) // https://github.com/rust-lang-nursery/futures-rs/commit/72d1203219a47242617cf8bead943773e641e696
		.map(|_sink:Sender<_>|()),
	).unwrap();
	println!("done {}", arg);
}

fn main() {
	init(Resources {
		mem: 20 * 1024 * 1024,
		..Resources::default()
	});
	let x = futures::executor::block_on(futures::future::join_all((0..10).map(|i| {
		let pid = spawn(
			sub,
			i,
			Resources {
				mem: 20 * 1024 * 1024,
				..Resources::default()
			},
		).expect("SPAWN FAILED");
		let (sender, receiver) = (
			Sender::<Option<String>>::new(pid),
			Receiver::<Option<String>>::new(pid),
		);
		futures::stream::iter_ok(vec![
			String::from("abc"),
			String::from("def"),
			String::from("ghi"),
			String::from("jkl"),
			String::from("mno"),
		]).forward(sender)
		.and_then(|(_stream,sink)|sink.close()) // https://github.com/rust-lang-nursery/futures-rs/commit/72d1203219a47242617cf8bead943773e641e696
		.map(|_sink:Sender<_>|())
			.join(
				receiver
					.fold(String::new(), |acc, x| futures::future::ok(acc + &x)),
			)
			.map(|(_, res)| res)
	}))).unwrap();
	println!("{:?}", x);
}
