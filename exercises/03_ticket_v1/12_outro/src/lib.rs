// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name : String,
    quantity : u32,
    unit_price : u32
}

impl Order {
    
    fn validate_product_name(product_name : String) -> String {
        if product_name.is_empty(){
            panic!("Product Name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product Name cannot be longer than 300 bytes");
        }
        product_name
    }
    
    fn validate_greater_zero(value : u32) -> u32 {
        if value == 0 {
            panic!("Cannot be greater than zero");
        }
        value
    }
    
    pub fn new(product_name : String, quantity : u32, unit_price: u32) -> Order {
        
        let product_name = Order::validate_product_name(product_name);
        let quantity = Order::validate_greater_zero(quantity);
        let unit_price = Order::validate_greater_zero(unit_price);
        
        Order {
            product_name,
            quantity,
            unit_price
        }
    }

    
    pub fn total(&self) -> u32 {
        &self.quantity * &self.unit_price
    }
    
    pub fn set_product_name(&mut self, new_product_name : String){
        self.product_name = Order::validate_product_name(new_product_name);
    }
    
    pub fn set_quantity(&mut self, new_quantity: u32){
        self.quantity = Order::validate_greater_zero(new_quantity);
    }

    pub fn set_unit_price(&mut self, new_unit_price : u32){
        self.unit_price = Order::validate_greater_zero(new_unit_price);
    }
    
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
}