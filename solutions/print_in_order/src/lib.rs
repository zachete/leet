use std::{sync::Mutex, thread, time::Duration};

struct Foo {
    step: Mutex<u8>,
}

impl Foo {
    fn new() -> Self {
        Foo {
            step: Mutex::new(0),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        loop {
            {
                let mut step = self.step.lock().unwrap();
                if *step == 0 {
                    *step = 1;
                    print_first();
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        loop {
            {
                let mut step = self.step.lock().unwrap();
                if *step == 1 {
                    *step = 2;
                    print_second();
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        loop {
            {
                let step = self.step.lock().unwrap();
                if *step == 2 {
                    print_third();
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}
