//ASSOCIATED CONSTANT - is a constant declared within the trait.
// a constant is a name for a fixed immutable value
// used to store fixed unchanging immutable data

/*
trait Taxable {
    const TAX_RATE: f64 = 0.25;

    // fn tax_bill(&self) -> f64;
    //tax_bill method is doing more or less similar things. so it is a candidate to be default implementation
    //but rust compiler will not be happy b/c the it does not know about the amount field. / what if another struct without amount fields comes
    //SO ONE SOLUTION IS TO USE A GETTER METHOD - is a method that retrieves a piece of data. it "gets" a piece of data from a type
    // fn tax_bill(&self) -> f64 {
    //     self.amount * Self::TAX_RATE
    // }

    fn amount(&self) -> f64; //getter method - which will give us the float value we need to calculate the tax_bill

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }

    //and now we want a general implementation to double the gain you got from any taxable income
    //but the following implementation will not work for the same reason we talked in the getter function usage
    //SO ONE SOLUTION IS USING SETTER METHOD - is a method that writes a piece of data. it "sets" a piece of state
    // fn double_amount(&mut self) {
    //     self.amount = self.amount() * 2.0;
    // }

    fn set_amount(&mut self, new_amount: f64); //setter method
    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0); //getter and setter combined
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    // fn tax_bill(&self) -> f64 {
    //     //but how to get the constant from the trait?
    //     // answer - we will use "Self"- uppercase S, which is the type
    //     self.amount * Self::TAX_RATE
    // }

    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

struct Bonus {
    value: f64,
}

impl Taxable for Bonus {
    //what is we have a different TAX_RATE for this?
    // answer: we can override it in this impl scope
    const TAX_RATE: f64 = 0.5;
    // fn tax_bill(&self) -> f64 {
    //     self.value * Self::TAX_RATE
    // }

    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

fn main() {
    let mut income: Income = Income { amount: 5000.00 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());

    let mut bonus: Bonus = Bonus { value: 5000.00 };
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
}
*/

/* SUPERTRAIT - is a trait from which another trait inhertis functionality.
- the parent is called the supertrait and the child is called the subtrait
-the child trait can inherit the behaviours from the parent trait and can also define its own requirements
-ANY TYPE IMPLEMENTS THE CHILD TRAIT MUST ALSO IMPLEMENT THE PARENT TRAIT
 */

//GIVE A FLEXIBILITY TO THE RETURNED THINGS FROM THE VARIABLE. FOR EG: SOME OF THEM MAY WANT TO RETURN A FLOAT FOR THE DOUBLE FUNCTION, BUT SOME MAY WANT TO RETURN INTEGER
trait Investment<T> {
    fn amount(&self) -> T; //getter method - which will give us the float value we need to calculate the tax_bill

    // fn set_amount(&mut self, new_amount: T); //setter method

    //we want to use generics, so let's comment out this function.b/c we use genercis , so the default implementation is not applicable as the multipyling value determines the type like 2, 2.0
    // fn double_amount(&mut self) {
    //     self.set_amount(self.amount() * 2.0); //getter and setter combined
    // }
    fn double_amount(&mut self);
}

// : -> means inherit
//so any type implements the taxable trait must also define methods in the parent trait "Investment"
trait Taxable: Investment<f64> {
    //it is enough for the generic to be f64 for this trait. so no need to use "T" here
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    // fn set_amount(&mut self, new_amount: f64) {
    //     self.amount = new_amount;
    // }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    // fn set_amount(&mut self, new_amount: f64) {
    //     self.value = new_amount;
    // }

    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;
    fn tax_bill(&self) -> f64 {
        self.value * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}
//WE CAN ONLY IMPLEMENT PARENT TRAITS
impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }
    // fn set_amount(&mut self, new_amount: f64) {
    //     self.minutes = new_amount;
    // }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    let mut income: Income = Income { amount: 5000.00 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());

    let mut bonus: Bonus = Bonus { value: 5000.00 };
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed: ${:.2}", bonus.tax_bill());

    let weekend = QualityTime { minutes: 120 };
    println!("Relaxation time : {:.2}", weekend.amount());
}
