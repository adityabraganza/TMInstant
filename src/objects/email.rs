pub struct Email{
    id: String,
    domain: String,
}
 
impl Email{
    pub fn new(id: &str, domain: &str) -> Email{
        return Email{id: id.to_string(), domain: domain.to_string()};
    }

    pub fn new_addr(address: &str) -> Email{
        let values: Vec<&str> = address.split("@").collect();
        return Email{id: values[0].to_string(), domain: values[1].to_string()};
    }

    pub fn get_id(&self) -> String {return self.id.clone()}

    pub fn get_domain(&self) -> String {return self.domain.clone()}

    pub fn get_email(&self) -> String {return self.get_id() + "@" + &self.get_domain()}

    pub fn copy(&self) -> Email {return Email{id: self.get_id(), domain: self.get_domain()}}
}

//I could have deleted this but im nice :) - A.C.