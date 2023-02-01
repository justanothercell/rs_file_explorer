use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use getch::Getch;

fn enable_virtual_terminal_processing() {
    #[cfg(windows)]
    {
        use winapi_util::console::Console;

        if let Ok(mut term) = Console::stdout() {
            let _ = term.set_virtual_terminal_processing(true);
        }
        if let Ok(mut term) = Console::stderr() {
            let _ = term.set_virtual_terminal_processing(true);
        }
    }
}

fn quit() -> ! {
    print!("\x1b[?25h");  // show cursor
    print!("\x1b[?47l");  // restore screen
    let _ = stdout().flush();
    exit(0)
}

pub(crate) struct Cli {
    input: Arc<Mutex<VecDeque<u8>>>,
}

impl Cli {
    pub(crate) fn start() {
        enable_virtual_terminal_processing();
        print!("\x1b[?25l");  // hide cursor
        print!("\x1b[?47h");  // save screen
        print!("\x1b[2J");   // erase screen
        let _ = stdout().flush();
        let mut cli = Self {
            input: Default::default()
        };
        let thread_in = cli.input.clone();
        thread::spawn(move ||{
            let getch = Getch::new();
            loop {
                let ch = getch.getch().unwrap();
                thread_in.lock().unwrap().push_back(ch);
            }
        });
        cli.run();
        ctrlc::set_handler(|| quit()).expect("Error setting Ctrl-C handler");
    }

    fn run(&mut self){
        loop {
            if let Some(c) = self.input() {
                self.handle_input(c)
            }
            // move to top left, print
            print!("\x1b[H{}", self.render())
        }
    }

    fn handle_input(&mut self, c: u8){
        match c {
            3 | 27 => quit(),  // ctrl-c | esc
            224 => match self.await_input() {
                72 => (),  // up
                80 => (),  // down
                75 => (),  // left
                77 => (),  // right

                71 => (),  // start
                79 => (),  // end
                73 => (),  // screen up
                81 => (),  // screen down
                _ => ()
            }
            _ => ()
        }
    }

    fn render(&self) -> String {
        let (w, h) = Self::size();
        let mut out = String::new();
        out.push_str("AAAAAAA\nAAAAAAAAAAAAAAA\nAAAAA");
        out
    }

    fn input(&self) -> Option<u8>{
        self.input.lock().unwrap().pop_front()
    }
    fn await_input(&self) -> u8{
        loop {
            if let Some(c) = self.input.lock().unwrap().pop_front() {
                return c
            }
        }
    }

    fn size() -> (usize, usize) {
        term_size::dimensions().expect("error getting size of terminal")
    }
}