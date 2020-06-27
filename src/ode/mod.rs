use ode_solvers::*;

type State = Vector3<f64>;
type Time = f64;

pub struct Replication<'a>{
    pub N: f64,
    pub act: &'a Vec<Vec<f64>>,
}

impl<'a> ode_solvers::System<State> for Replication<'a>{
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let capa: f64 = 1. - y.iter().sum::<f64>()/self.N;

        for i in 0..3{
            dy[i] = (self.act[i][0]*y[0] + self.act[i][1]*y[1]) * y[i] * capa;
        }
    }
}
