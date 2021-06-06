pub fn run () {
    // Print to console
    println!("Hello from print.rs file");
    println!("{} and {} plus, the sum is {}",10,20,10+20);
    
    //Basic formatting
    println!("{} is from {}","Umar","karachi");

    //positional arguments
    println!("{0} is from {1} and {0} like {2}","Umar","karachi","Mangoes");

    //Named arguments
    println!("{name} like to play {activity}",name="Umar",activity="cricket");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o} ",10,10,10);

     //Placeholder for Debug trait
     println!("{:?}" ,(12,true,"Hafsa"));

}
