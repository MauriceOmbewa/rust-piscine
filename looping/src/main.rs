use std::io;

fn main(){
    let mut count = 0;
    // let mut fininput = "wrong";
    
    loop{
        let mut input = String::new();
        count += 1;

        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        if input.trim() == "The letter e"{
            break;
        } else {
            continue;
        }
    }

    println!("Number of trials: {}", count);
}