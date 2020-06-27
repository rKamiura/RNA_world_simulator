mod world;


fn main(){
    let act = vec![
        vec![0.09, 0.07, 0.],
        vec![0.09, 0.09, 0.],
        vec![0.48, 0.01, 0.]
    ];

    let w = world::creation(
        100, 3000, 60., 4, 5000, vec![30., 30.], 0., act, 15., 1
    );
    w.paralleling();

    println!("💕");
}