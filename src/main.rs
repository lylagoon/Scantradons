use rand::thread_rng;


fn raw_input(prompt: &str) -> String{
	use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("{}", prompt);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}

// 'from' is how much person being taken for has 'amt' is what 'to' want to take
// takes amt from 'from', returns how much it took, up to amt, can take less if 
// not enought to take
fn take(from: &mut i32, amt: i32) -> i32{
	if *from < amt{
		let t  = *from;
		*from = 0;
		return t;
	}
	*from = *from - amt;
	return amt;
}

//tries to move value from 'from' to 'to', if 'from' does not have enought
//whaterver 'from' has, is transfered to 'to'
fn transfer(from: &mut i32, to: &mut i32, amt: i32){
	//from is person who has     to person who wants     amt how much to wants
	*to += take(from, amt); 
}
const FARM_COST: i32 = 25;
const FOOD_PRICE: i32 = 1;
const FOOD_PER_PERSON: i32 = 1;
const FOOD_PER_FARM: i32 = 4;
const BABIES_PER_POP: f32 = 0.1;
const HOSPITAL_BABY_SURVIVE_BONUS: f32 = BABIES_PER_POP/3.0;
const ARMY_COST: i32 = 19;
const BUILDING_COST: i32 = 50;



#[derive(Debug)]
struct Village{
	name: String,
	pop: i32,
	money: i32,
	farms: i32,
	food_per_farm: i32,
	hospitals: i32,
    granarys: i32,
    walls: i32,
}

impl Village{
	fn new(name: String) -> Village{
		return Village{name: name, 
			pop: 2, 
			money: 50, 
			farms: 1, 
			food_per_farm: FOOD_PER_FARM,
			hospitals: 0,
		    granarys: 0,
		    walls: 0,
		};
	}
}

struct Gov{
	money: i32,
	tax_rate: i32,
}

fn update(gov: &mut Gov, vil: &mut Village){
	let req_food = FOOD_PER_PERSON * vil.pop;
	let made_food = vil.farms * FOOD_PER_FARM;
	let rem_food = made_food - req_food;
	if rem_food >= 0{ // we have leftovers
		vil.money += rem_food * FOOD_PRICE;
		if vil.money > 20{
			vil.pop += (vil.pop as f32 * BABIES_PER_POP).ceil() as i32;
		}
	} else { //not enough food
		let starving = (rem_food.abs() - 1)/FOOD_PER_PERSON + 1;
		take(&mut vil.pop, starving);// send pepole to village with extra food?
	}
	println!("Rem food {:?}", rem_food);
	let tax = vil.pop * gov.tax_rate;
	println!("tax {:?}", tax);
	println!("vill money {:?}", vil.money);
	transfer(&mut vil.money, &mut gov.money, tax);
	if vil.money > FOOD_PRICE{
		vil.farms += 1;
	}
	println!("You have {:?} gold coins.",  gov.money);
	println!("Village state:\n {:?}", vil);
}



fn governer(gov: &mut Gov, vil: &mut Village) -> bool{

	let gove_ds = raw_input("What is the tax rate: ");
	let asnum: Result<i32, _> = gove_ds.parse();
    match asnum{
        Ok(n) => {
        	gov.tax_rate = n;
        },
        Err(_) => println!("Thats not a number")
    }

    let army = raw_input("do you want to build an army: ");
    if army == ("yes"){//not sure how army will help
    	gov.money -= ARMY_COST
    
    }else{
    	println!("you said {:?} the consequenses are unkown", army);
    }
    
    let cat = raw_input ("would you like to build something to assist in production or saftey: ");
    if cat == "yes"{
    		let build = raw_input("do you want to build a granary wall or hospital: ");
    			if build == ("granary"){
    				vil.granarys += 1;
    				gov.money -= BUILDING_COST;
    				vil.food_per_farm += 2;

    				//famines will go down
    			}
    			if build ==("hospital"){
    				vil.hospitals += 1;
    				gov.money -= BUILDING_COST;
    				if vil.hospitals >= 1  && (vil.pop >= 2){
						vil.pop += vil.hospitals * 4;//may change
					//sickness will go down
					}
				}
				if build ==("wall"){
					vil.walls += 1;
					gov.money -= BUILDING_COST;
					//raids go down
				}
    				
    }else{
    	println!("you said {:?} the consequenses are unkown", cat)
    }
    return false;

}	

//sick famine raids
fn promblems(vil: &mut Village, gov: &mut Gov) {

	let mut rng = thread_rng();

	let famine: u32 = rng.gen_range(1, 95); 
	if famine == 36{
		vil.food_per_farm -= rng.gen_range(1, 20);
		if vil.food_per_farm < 0{
			vil.food_per_farm = 0;
		}
		vil.farms -= 1;
		//for random amount of turns 1-7
		println!("A famine has hit")

	}

	//sick = random number beetween 1-70;
	//if sick == 22;{
		//vil.pop -= randomly beetween 1-30;
		//for random amount of turns 1-6;
		//println("your people are experiencing sickness");

	//}else{
		//do nothing
	//}

	//raid = random number beetween 1-150;
	//if raid == 79;{
		//vil.money -= random btw 1-40;
		//gov.money -= rdm btw 1-60;
		//if NUM_OF_BUILDINGS > (1){
			//NUM_OF_BUILDINGS -= rdm btw 1-3
		//}
		//println("you have been raided");
	//}else{
		//do nothing
	//}



}


fn main() {
    println!("Hello, world!");

    let mut g: Gov = Gov{money: 0, tax_rate: 1};
    let mut v: Village = Village::new(String::from("Valanthar"));

    let mut exit = false;
    while !exit {
    	update(&mut g, &mut v);
    	exit = governer(&mut g, &mut v);
    }
}




