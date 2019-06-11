// use piston_window::*;
use std::net::TcpListener;

fn main() {
    let x = 1; // integer
    let y = 2.0; //floating point number
    let dynamic = 8 * 8;
    let myarray = [1,2,3,4,5,6,7,8];
    // let tuple = (1,2,"up");
   // let (hi, bye, fry) = tuple; //using tuple to create variables on the fly

    hello_world("Chrisps");
    add(5, 3);
    println!("hello{}{} {} {}", x, y, dynamic, myarray[5]);

    // let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     println!("a connection was made!!")
    // }

    for fig in myarray.iter() {
        println!("{}",fig)
    }

}

 fn hello_world(name: &str) {
     println!("hello your {}",name)

 }

 fn add(a:i8, b:i8){
     println!("{}",a + b)
 }

