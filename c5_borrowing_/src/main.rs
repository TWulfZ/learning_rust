// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding References

// Mutable Reference.
// Create Reference by add "&"

// -I- Immutable Reference
// fn main() {
//     let x = 5;
//     let r = &x; 

//     println!("The value of x is: {}", x);
//     println!("The value of r is: {}", r);
// }

// -II- Mutable Reference
// fn main() {
//     let mut x = 5;
//     let r = &mut x; 
//     *r += 1;
//     *r -= 3;

//     println!("The value of x is: {}", x);
//     println!("The value of x is: {}", r);

// }

// Demostration on one mutable reference or many immutable references
fn main() { 
    let mut account = BankAccount {
        owner: String::from("TWulfZ"),
        balance: 100.0
    };
    // Immutable borrow to check the balance
    account.check_balance();
    
    // Mutable borrow to withdraw money
    account.withdraw(69.9);
    // Immutable borrow to check the balance
    account.check_balance();


}


struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdarwing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}