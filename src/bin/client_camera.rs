use std::env;
use sentinel::db::{create_camera, query_camera, establish_connection};

fn help() {
    println!("subcommands");
    println!("\tnew <name,address>: create a new camera");
    println!("\tshow:               show all cameras");
}

fn new_camera(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <name,address>");
        help();
        return;
    }

    let conn = establish_connection();
    create_camera(&conn, &args[0], &args[1]);
}

fn show_cameras(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("CAMERAS\n-----");
    for camera in query_camera(&conn) {
        // match camera.name {
        //     Some(x) => println!("Camera name: {}", x),
        //     None => println!("This camera has no name"),
        // }
        // match camera.address {
        //     Some(x) => println!("Camera address: {}", x),
        //     None => println!("This camera has no address"),
        // }
        match (camera.name, camera.address) {
            (Some(x), Some(y)) => println!("{}, {}", x, y),
            (Some(x), None) => println!("{}, no address found", x),
            (None, Some(y)) => println!("no name found, {}", y),
            (None, None) => println!("no name or address found"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_camera(&args[2..]),
        "show" => show_cameras(&args[2..]),
        _ => help(),
    }
}