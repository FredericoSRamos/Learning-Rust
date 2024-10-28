use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use rand;

const QTD_FILOSOFOS: usize = 5;

const PENSANDO: u8 = 0;
const COM_FOME: u8 = 1;
const COMENDO: u8 = 2;

struct Mesa {
    estado: Mutex<[u8; QTD_FILOSOFOS]>,
    hashis: Mutex<[bool; QTD_FILOSOFOS]>,
    cv: [Condvar; QTD_FILOSOFOS],
}

impl Mesa {
    fn new() -> Self {
        Mesa {
            estado: Mutex::new([PENSANDO; QTD_FILOSOFOS]),
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

    fn pegar_hashis(&self, id: usize) {
        let mut hashis = self.hashis.lock().unwrap();
    
        let left = id;
        let right = (id + 1) % QTD_FILOSOFOS;
    
        while !hashis[left] || !hashis[right] {
            if !hashis[left] {
                hashis = self.cv[left].wait(hashis).unwrap();
            } else {
                hashis = self.cv[right].wait(hashis).unwrap();
            }
        }
    
        hashis[left] = false;
        hashis[right] = false;
    }
    
    fn devolver_hashis(&self, id: usize) {
        let mut hashis = self.hashis.lock().unwrap();

        let left = id;
        let right = (id + 1) % QTD_FILOSOFOS;

        hashis[left] = true;
        hashis[right] = true;
    }
    
    fn comer(&self, id: usize) {
        
        let mut estado = self.estado.lock().unwrap();
        estado[id] = COM_FOME;
        println!("Filósofo {} com fome", id + 1);
        drop(estado);
    
        self.pegar_hashis(id); 
    
        let mut estado = self.estado.lock().unwrap();
        estado[id] = COMENDO;
        println!("Filósofo {} comendo", id + 1);
        drop(estado);
    
        thread::sleep(Duration::from_secs(1 + rand::random::<u64>() % 5));
    
        self.devolver_hashis(id);
    
        let mut estado = self.estado.lock().unwrap();
        estado[id] = PENSANDO;
        println!("Filósofo {} pensando", id + 1);
        drop(estado);
    
        self.cv[id].notify_one(); 
        self.cv[(id + 1) % QTD_FILOSOFOS].notify_one();
    }
}

fn main() {
    let mesa = Arc::new(Mesa::new());

    let philosophers: Vec<_> = (0..QTD_FILOSOFOS)
        .map(|i| {
            let mesa = mesa.clone();
            thread::spawn(move || {
                println!("Filósofo {} pensando", i + 1);
                loop {
                    thread::sleep(Duration::from_secs(1 + rand::random::<u64>() % 5));
                    mesa.comer(i);
                }
            })
        }).collect();

    for philosopher in philosophers {
        philosopher.join().unwrap();
    }
}