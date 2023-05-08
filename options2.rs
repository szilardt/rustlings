fn simple_option() {
    let target = "rustlings";
    let optional_target = Some(target);

    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_target {
        assert_eq!(word, target);
        println!("\nword : {} ; optional_target.unwrap(): {}", word, optional_target.unwrap() );
    }
}

fn layered_option() {
    let mut range = 10;
    let mut optional_integers: Vec<Option<i8>> = Vec::new();
    for i in 0..(range + 1) {
        optional_integers.push(Some(i));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(integer) = optional_integers.pop().unwrap(){
        assert_eq!(integer, range);
        if range==0{
            println!("range = {:?} -> break the while loop", range );
            break;
        }
        range -= 1;
    }
}


fn main(){
    simple_option();
    layered_option();
}







