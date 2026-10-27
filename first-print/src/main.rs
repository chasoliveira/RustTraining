fn main() {
    let mut name = String::from("Chaso");
    name.push_str(" - Chasoliveira");
    print_name(&name);
    immutable();

    let mut account = BankAccount {
        owner: "Charles".to_string(),
        balance: 150.5,
    };

    //Immutable borrow
    account.check_balance();

    //Mutable borrow
    account.withdraw(45.05);

    //Immutable borrow
    account.check_balance();

    variables();
    constants();
    shadowing();
}

fn print_name(name: &String) {
    println!("\n\tMy name is {}\n", &name);
}

//-- Immutable Reference

fn immutable() {
    let mut _x = 5;
    let _r = &mut _x;

    *_r += 1;
    println!("Value of _x: {}", _x);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        );
    }
}

//---------
//-- Variables references

fn variables() {
    println!("----- Variables References --------");

    let mut a = 5;
    println!("The value of a is {}", a);
    a = 10;
    println!("The value of a is now {}", a);
    println!();
}

// --- Contants

const PI: f64 = 3.1415592653;
const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;
fn constants() {
    println!("---- Constants ---");
    const Y: i32 = 10;

    println!(
        "The Constants does not use `mut` and must be UPPER CASE: {}",
        Y
    );

    println!("The PI values is {}", PI);
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);
}

// -- shadowing
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x in main func is {x}");
}
