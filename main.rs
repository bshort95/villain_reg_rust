use std::io;

pub struct Villain {
    name: String,
    power: String,
    age: u16,
    crime: String,
  
}

fn main()
{
    let mut vill_list: Vec<Villain> = Vec::new();
    //here we hardcode some villains
    let bob = Villain {name:"bob".to_string(), power:"evil-genius".to_string(), age:65, crime:"stole all of the left shoes".to_string()};
    let walter = Villain {name:"walter".to_string(), power:"fire control".to_string(), age:22, crime:"he did start the fire".to_string()};
    vill_list.push(bob);
    vill_list.push(walter);

    let mut input = String::new();
    let mut fs = true;

    println!("welcome to the evil villian registry");
    
    while fs
    {
        menu();
        input.clear();
        io::stdin().read_line(&mut input).expect("wrong format");
        let user = input.trim();

        if user == "1"
        {
            display(&vill_list);
        }
        else if user == "2"
        {
            add(&mut vill_list);            
        }
        else if user == "3"
        {
            remove_villain(&mut vill_list);
        }
        else if user == "4"
        {
            fs = false;
        }
        else
        {
            println!("please enter a appropriate option");
        }

    }
}

pub fn add(v: &mut Vec<Villain>)
{

    let mut input_name = String:: new();
    let mut input_power = String:: new();
    let mut input_age = String:: new();
    let mut input_crime = String:: new();

    println!("enter the villains name");
    input_name.clear();
    io::stdin().read_line(&mut input_name).expect("wrong format");
    let name = input_name.trim();

    println!("enter the villains power");
    input_power.clear();
    io::stdin().read_line(&mut input_power).expect("wrong format");
    let power = input_power.trim();

    let mut fs = true;
    let mut age = 0;
    
    while fs
    {
        println!("enter the villains age");
        input_age.clear();
        io::stdin().read_line(&mut input_age).expect("wrong format");

        age = input_age.trim().parse().unwrap_or(0);

        if age == 0
        {
            println!("please enter an appropiate answer")
        }
        else
        {
            fs = false;
        }   
    }

    println!("enter the villains crime");
    input_crime.clear();
    io::stdin().read_line(&mut input_crime).expect("wrong format");
    let crime = input_crime.trim();

    let temp = Villain{name:name.to_string(), power:power.to_string(), age:age, crime:crime.to_string() };
    v.push(temp);

}

pub fn display(vil: &Vec<Villain>)
{
    println!("there are {} villain(s) on record", vil.len());
    
    for i in 0..vil.len()
    {
        println!("----------------------------");   
        println!(" villian name: {}",vil[i].name);
        println!(" super power: {}",vil[i].power);
        println!(" age: {}",vil[i].age);
        println!(" crime: {}",vil[i].crime);
        println!("----------------------------");
    }

}

pub fn remove_villain(v: &mut Vec<Villain>)
{

    
    let mut input = String:: new();
    
    println!("what is the name of the villain you are going to remove");
    io::stdin().read_line(&mut input).expect("wrong format");
    let answer = input.trim();
    let mut index = v.len() + 1;
    for i in 0..v.len()
    {
        if answer == v[i].name
        {
            index = i;
        }  
    }

    if index > v.len()
    {
        println!("im sorry but the name {} was not found", answer)
    }
    else
    {
        v.remove(index);

    }


}


pub fn menu(){

    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("press 1 to display the registry");
    println!("press 2 to add a villian to the registry");
    println!("press 3 to remove a villian from the registry");
    println!("press 4 to quit");
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    
}