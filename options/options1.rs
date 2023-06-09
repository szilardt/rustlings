// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

//I AM NOT DONE

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: i8) -> Option<i8> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    match time_of_day {
    	0..=21  => Some(5),
    	22 | 23 => Some(0),
    	_ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5);
    }
}

fn main() {
    
  if let Some(x)=maybe_icecream(24){
    println!("output for maybe_icecream(24) = {}",x);
    }

    for i in -2..26{
        match maybe_icecream(i){
            Some(j) => println!("maybe_icecream({}) = {:?} icereams avilable", i, j ),
            None    => println!("maybe_icecream({}) = None", i ),
        }
    }

}
