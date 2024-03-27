use yt_subs::get;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let (subs, total) = get(&args);
    match args.len() {
        0 => println!("No channels provided"),
        1 => {
            println!("Subs for {}: {}", args[0], subs[0]);
        }
        _ => {
            for channel in args.iter().zip(subs.iter()) {
                println!("Subs for {}: {}", channel.0, channel.1);
            }
            println!("Total: {}", total);
        }
    }
}
