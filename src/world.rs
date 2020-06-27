use chrono::Utc;
use std::thread;
use std::sync::{Mutex, Arc};
use rand::seq::SliceRandom;
use rand_distr::{Binomial, Distribution};
use ode_solvers::dopri5::*;
use ode_solvers::*;

use RNA_world_simulator::ode;

type State = Vector3<f64>;
type Time = f64;

pub struct World{
    pub start: String,
    R: usize,
    C: usize,
    N: f64,
    D: u8,
    A: usize,
    init_h: Vec<f64>,
    init_p: f64,
    act: Vec<Vec<f64>>,
    RT: f64,
    trial: u8,
}

impl World{
    pub fn paralleling(&self){
        
        for _ in 0..self.trial{
            self.chronicle()
        }

    }

    fn chronicle(&self){
        // Mapping indexes
        let diluted = dilution_mapper(self.R, self.C, self.D);
        let agitated = agitation_mapper(self.R, self.C, self.A);

        //
        let mut rng = rand::thread_rng();
        let ks = Arc::new(Mutex::new( self.act.clone() ));

        // Lives initialization
        let mut lives: Vec<Vec<f64>>
            = (0..self.C).map(|x| vec![self.init_h[0], self.init_h[1], self.init_p] ).collect();

        // Main loop
        for r in 0..self.R{
            println!("Round {}", r+1);

            // Dilution
            for &d in diluted[r].iter(){ lives[d] = vec![0., 0., 0.] }

            // Agitation
            for a in agitated[r].iter(){
                break;

                for i in 0..3{
                    let sum = lives[a[0]][i]+lives[a[1]][i];
                    let divider = Binomial::new(sum as u64, 0.5).unwrap();

                    lives[a[0]][i] = divider.sample(&mut rng) as f64;
                    lives[a[1]][i] = sum - lives[a[0]][i];
                }
            }

            // Replication
            let vecs: Vec<Arc<Mutex<Vec<f64>>>>
                = (0..self.C).map(|c| Arc::new(Mutex::new( lives[c].clone() ))).collect();
            let mut handles = vec![];
            for c in 0..self.C{
                let mut vec = Arc::clone(&vecs[c]);
                let mut k = Arc::clone(&ks);

                let handle = std::thread::spawn(move || {
                    let mut v = vec.lock().unwrap();

                    //let system
                    //    = ode::Replication{ N: self.N, act: &k.lock().unwrap() };
                    let y0 = State::new(v[0], v[1], v[2]);
                    
                    *v = vec![0., 0., 0.];

                    println!("{:?}", k.lock().unwrap());
                });
                handles.push(handle);



                //let system = ode::Replication{ N: self.N, act: self.act.clone() };
                //let y0 = State::new(lives[c][0], lives[c][1], lives[c][2]);
//
                //let mut stepper = Dopri5::new(system, 0.0, self.RT, 60.0, y0, 1.0e-10, 1.0e-10);
                //let res = stepper.integrate();
            }

            for handle in handles.into_iter() {
                let _ = handle.join().unwrap();
            }

            println!("{:?}", ks);


        }

    }
}

pub fn creation(
    R: usize, C: usize, N:f64, D: u8, A: usize, init_h: Vec<f64>, init_p: f64,
    act: Vec<Vec<f64>>, RT: f64, trial: u8) -> World{
    World{
        start: Utc::now().format("%Y%m%d%H%M%S%f").to_string(),
        R,
        C,
        N,
        D,
        A,
        init_h,
        init_p,
        act,
        RT,
        trial,
    }
}

fn dilution_mapper(R: usize, C: usize, D: u8) -> Vec<Vec<usize>>{
    let rate = ((C as f32) * (1. - 1./(D as f32))) as usize;
    let mut diluted: Vec<Vec<usize>> = Vec::with_capacity(R);
    let mut rng = rand::thread_rng();

    for _ in 0..R{
        let mut temp = (0..C).collect::<Vec<usize>>();
        temp.shuffle(&mut rng);

        diluted.push(temp.into_iter().take(rate).collect());
    }

    diluted
}

fn agitation_mapper(R: usize, C: usize, A: usize) -> Vec<Vec<Vec<usize>>>{
    let mut agitated: Vec<Vec<Vec<usize>>> = Vec::with_capacity(R);
    let mut rng = rand::thread_rng();

    let temp: Vec<usize> = (0..C).collect();
    for _ in 0..R{
        let mut ag: Vec<Vec<usize>> = Vec::with_capacity(A);
        
        for _ in 0..A{
            let comps: Vec<_> = temp.choose_multiple(&mut rng, 2).cloned().collect();

            ag.push(comps);
        }
        agitated.push(ag);
    }

    agitated
}
