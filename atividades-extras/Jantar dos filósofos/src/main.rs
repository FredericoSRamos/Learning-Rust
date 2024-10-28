use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use rand;

const QTD_FILOSOFOS: usize = 5;

const PENSANDO: u8 = 0;
const COM_FOME: u8 = 1;
const COMENDO: u8 = 2;

struct DiningPhilosophers {
    hashis: Mutex<[bool; QTD_FILOSOFOS]>,
    cv: [Condvar; QTD_FILOSOFOS],
}

impl DiningPhilosophers {
    fn new() -> Self {
        DiningPhilosophers {
            hashis: Mutex::new([true; QTD_FILOSOFOS]),
            cv: [
                Condvar::new(),
                Condvar::new(),
                Condvar::new(),
                Condvar::new(),
                Condvar::new(),
            ],
        }
    }

    fn comer(&self, id: usize, estado: &mut [u8; QTD_FILOSOFOS]) {
        println!("Filósofo {} com fome", id + 1);
        estado[id] = COM_FOME;

        let mut hashis = self.hashis.lock().unwrap();

        let left = id;
        let right = (id + 1) % QTD_FILOSOFOS;

        while !hashis[left] || !hashis[right] {
            hashis = self.cv[id].wait(hashis).unwrap();
        }

        hashis[left] = false;
        hashis[right] = false;
        estado[id] = COMENDO;
        drop(hashis);

        println!("Filósofo {} comendo", id + 1);
        thread::sleep(Duration::from_secs(1 + rand::random::<u64>() % 5));

        hashis = self.hashis.lock().unwrap();
        hashis[left] = true;
        hashis[right] = true;
        drop(hashis);

        self.cv[left].notify_one();
        self.cv[right].notify_one();
    }
}

fn main() {
    let dining_philosophers = Arc::new(DiningPhilosophers::new());
    let mut states = [0; QTD_FILOSOFOS];

    let philosophers: Vec<_> = (0..QTD_FILOSOFOS)
        .map(|i| {
            let dining_philosophers = dining_philosophers.clone();
            thread::spawn(move || {
                loop {
                    println!("Filósofo {} pensando", i + 1);
                    states[i] = PENSANDO;
                    thread::sleep(Duration::from_secs(1 + rand::random::<u64>() % 5));
                    dining_philosophers.comer(i, &mut states);
                }
            })
        }).collect();

    for philosopher in philosophers {
        philosopher.join().unwrap();
    }
}