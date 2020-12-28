use chrono::prelude::*;

struct Customer {
  id: String,
  address: Address,
  firstname: String,
  secondname: String,
  phonenumber: String,
  email: String,
  ebill: Vec<String>,
  date_created: DateTime<Utc>,
  notes: String,
  created_by: String,
}
impl Customer {
 fn new(id, address, firstname, secondname, email, phone, ebillrange, date_created, created_by) {
   Self {
    id,
    address,
    firstname,
    secondname,
    email,
    phone,
    ebillrange,
    date_created,
    created_by,
   }   
 }
}

struct Address{
 street: String,
 city: String,
 zip: String,
}
impl Address {
 fn new(strt: String, cty: String, zp: String) -> Self{
   Self {
     street: strt,
     city: cty,
     zip: zp
     
   }
 }
}


fn main(){
  
 
let street = String::from("564 applebee st");
let city = String::from("huntsville");
let zip = String::from("77396");
let firstname = String::from("Andy");
let lastname = String::from("Woodsmith");
let phone = String::from("225-125-421");
let email = String::from("andy.smith@gmail.com");
let ebillrange = vec!["0-40","50-90","100-140","150-190","200-240","250-290","300-340","350-400"];

let createdby = String::from("Bill Miyer");
let notes = String::from("Good roof");
 
let mut id: String;

   loop {
     id = id::generate_alphanumeric(7); // e.g.: 4rf6r7f
     
 
// Create customer object
   let new_customer = Customer::new(
      
      id,
      Address::new(city, street, zip),
      firstname,
      secondname,
      email,
      phone,
      ebillrange,
      notes,
 
      date_created: DateTime<Utc>,
      created_by,
      
   );
}

