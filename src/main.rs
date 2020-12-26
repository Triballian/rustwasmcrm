use chrono::{Datelike, Local, Utc};

struct Customer {
  id: String,
  address: String,
  firstname: String,
  secondname: String,
  phonenumber: String,
  email: String,
  ebill: Ebillrange,
  date_created: DateTime<Utc>,
  notes: String,
  created_by: String,
}
impl Customer {
 fn new {
   
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
struct Ebillrange {"0-40","50-90","100-140","150-190","200-240","250-290","300-340","350-400"}

fn main(){
  
 let street = String::from("564 applebee st");
 let city = String::from("huntsville");
 let zip = String::from("77396");
 let firstname = String::from("Andy");
 let lastname = String::from("Woodsmith");
 let email = String::from("andy.smith@gmail.com");
 let createdby = String::from("Bill Miyer");
 let notes = String::from("Good roof");
 
 let mut id: String;

   loop {
     id = id::generate_alphanumeric(7); // e.g.: 4rf6r7f
     if self.is_id_available(&id).await {
       break;
     }
 
// Create customer object
   let new_customer = Customer::new(
     id,
     customer::Address::new(city, street, zip),
     firstname,
     secondname,
     email,
     phone,
     date_created,
     notes,
     created_by,
     date_created: DateTime<Utc>,
   );
}

