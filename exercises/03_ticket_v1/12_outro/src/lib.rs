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

struct Order {
    product_name: String, 
    quantity: u64, 
    unit_price: u16,
}

impl Order {

    //Implemnet methods to validate the setters and the new constructor.
    
    fn todo!()

    fn product_name(&self) -> &String {
        &self.product_name
    }

    fn quantity(&self) -> &u64 {
        &self.quantity
    }

    fn unit_price(&self) -> &u16 {
        &self.unit_price
    }

}