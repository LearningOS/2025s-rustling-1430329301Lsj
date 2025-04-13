// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    // ($ckl:expr) =>{
    //     println!("My macro called `{:?}`", $ckl);
    // };
    // ($($vals:expr),+) => {
    //     let mut time = 0;
    //     $(
    //     println!("My macro called `{:?},{}`", $vals, time);
    //     time += 1;
    //     )+
    // };
}

fn main() {
    my_macro!(); 
    // my_macro!("hhhhh", [1, 2, 3]);
    // my_macro!("hhh");
}
