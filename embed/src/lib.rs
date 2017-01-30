
use std::thread;

#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(||{
            let mut x = 0;
            for _ in 0..5_000_000 {
                x+=1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("thread finished with count = {}", h.join().map_err(|_| "could not join a thread").unwrap());
    }
}


//#[cfg(test)]
mod tests {
    //#[test]
    fn it_works() {
    }
}
