//= {
//=   "output": {
//=     "2": [
//=       "thread 'main' panicked at 'qwertyuiop', test/src/bin/d\\.rs:[0-9]+:[0-9]+\n",
//=       true
//=     ],
//=     "1": [
//=       "",
//=       true
//=     ]
//=   },
//=   "children": [],
//=   "exit": {
//=     "Right": "SIGABRT"
//=   }
//= }

#![deny(warnings, deprecated)]
extern crate deploy;
extern crate nix;
use deploy::*;
use std::{panic, process, thread, time};

fn main() {
	init(Resources {
		mem: 20 * 1024 * 1024,
		..Resources::default()
	});
	panic::set_hook(Box::new(|info| {
		eprintln!("thread '{}' {}", thread::current().name().unwrap(), info);
		let err = unsafe {
			nix::libc::setrlimit64(
				nix::libc::RLIMIT_CORE,
				&nix::libc::rlimit64 {
					rlim_cur: 0,
					rlim_max: 0,
				},
			)
		};
		assert_eq!(err, 0);
		process::abort()
	}));
	thread::sleep(time::Duration::new(0, 100_000_000));
	panic!("qwertyuiop");
}
