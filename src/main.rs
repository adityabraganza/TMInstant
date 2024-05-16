/*
To Do:
- Add option to load a custom email
- Finish functions
- View based on html, save to html file for better viewing
*/

mod objects;
use crate::objects::email::Email;
use reqwest::{self, Response};
use std::io;
use tokio;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EmailInfo{
    id: u32,
    from: String,
    subject: String,
    date: String
}

impl EmailInfo{
    fn get_id(&self) -> u32 {return self.id;}
    fn get_from(&self) -> String {return self.from.to_string();}
    fn get_subject(&self) -> String {return self.subject.to_string();}
    fn get_date(&self) -> String {return self.date.to_string();}
    fn to_string(&self) -> String {return 
        "id: ".to_string() + &self.get_id().to_string() + 
        "from: " + &self.get_from() + 
        "subject: " + &self.get_subject() + 
        "date: " + &self.get_date()
    }
}

#[derive(Deserialize, Debug)]
struct FullEmail{
    id: u32,
    from: String,
    subject: String,
    date: String,
    attachments: Vec<Attachment>,
    body: String,
    textBody: String,
    htmlBody: String
}

impl FullEmail{
    fn get_id(&self) -> u32 {return self.id;}
    fn get_from(&self) -> String {return self.from.to_string();}
    fn get_subject(&self) -> String {return self.subject.to_string();}
    fn get_date(&self) -> String {return self.date.to_string();}
    fn get_attachments(&self) -> &Vec<Attachment> {return &self.attachments;}
    fn get_body(&self) -> String {return self.body.to_string();}
    fn get_textbody(&self) -> String {return self.textBody.to_string();}
    fn get_htmlbody(&self) -> String {return self.htmlBody.to_string();}
    fn attachments_as_string(&self) -> String{
        let mut return_string: String = "".to_string();
        for attachment in &self.attachments{
            return_string = return_string + &attachment.to_String() + "\n";
        }

        return return_string;
    }
    fn to_string(&self) -> String {return 
        "id: ".to_string() + &self.get_id().to_string() + "\n" +
        "from: " + &self.get_from() + "\n" + 
        "date: " + &self.get_date() + "\n" + 
        "subject: " + &self.get_subject() + "\n" +
        "body: \n" + &self.get_textbody() + "\n" + 
        "attachments: " +&self.attachments_as_string()
    }
}

#[derive(Deserialize, Debug)]
struct Attachment{
    filename: String,
    contentType: String,
    size: u32
}

impl Attachment{
    fn get_filename(&self) -> String{return (&self.filename).to_string()}
    fn get_contentType(&self) -> String{return (&self.contentType).to_string()}
    fn get_size(&self) -> u32{return self.size}
    fn to_String(&self) -> String{return
        "filename: ".to_string() + &self.get_filename() + "\n" +
        "content type: " + &self.get_contentType() + "\n" + 
        "size: " + &self.get_size().to_string() + "\n"
    }
}

#[tokio::main]
async fn main(){
    let mut should_run = true;
    let mut input:String;
    let mut emails: Vec<Email> = Vec::new();
    clear_terminal();
    help();

    while should_run{
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let command: Vec<&str> = input.split("=").collect();

        match command[0]{
            "exit" => {
                clear_terminal();
                should_run = false;
            },

            "new" => {
                let response = get_api_data("https://www.1secmail.com/api/v1/?action=genRandomMailbox&count=1").await;

                match response {
                    Ok(response) => {
                        let response_text = response.text().await;

                        match response_text{
                            Ok(response_text) => {
                                let response_split: Vec<&str> = response_text.split("@").collect::<Vec<&str>>();
                
                                let id_array: Vec<&str> = response_split[0].split("\"").collect();
                                let email_array: Vec<&str> = response_split[1].split("\"").collect();
                                
                                let mut elements: Vec<String> = Vec::new();
                            
                                elements.push(id_array[1].to_string());
                                elements.push(email_array[0].to_string());
                                
                                emails.push(Email::new(&elements[0], &elements[1]));
                            
                                print!("{}", response_split[0].to_owned() + "@");
                                println!("{}", response_split[1]);
                                println!();
                            },
                            Err(err) => {
                                println!("There was an error with converting the API response while making a new email");
                                print!("Error: ");
                                println!("{}", err.to_string());
                                println!();
                            }
                        }
                    },
                    Err(err) => {
                        println!("There was an error with the API while making a new email");
                        print!("Error: ");
                        println!("{}", err.to_string());
                        println!();
                    },
                }
            },

            "check_all" => {
                println!();
                for email in &emails{
                    print_emails_from_id(email.copy()).await;
                }
            },

            "check_email_id" => {
                if command.len() > 1{
                    print_emails_from_id(Email::new_addr(command[1])).await;
                } else{
                    let mut response_print_emails_from_id = String::new();
                    io::stdin().read_line(&mut response_print_emails_from_id).unwrap();
                    response_print_emails_from_id = response_print_emails_from_id.trim().to_string();
                    print_emails_from_id(Email::new_addr(&response_print_emails_from_id)).await;
                }
            },

            "check_email" => {
                let mut email_id_to_check: u32 = 1;
                let mut email_id_to_check_string = "".to_string();
                let mut email_address: Email = Email::new("abc", "@example.com");
                let mut has_input = false;

                if command.len() > 1{
                    let inputs: Vec<&str> = command[1].split(",").collect();
                    if inputs.len() > 1{
                        has_input = true;
                        email_id_to_check = inputs[0].parse::<u32>().unwrap();
                        email_address = Email::new_addr(inputs[1]);
                    }
                }

                if !has_input{
                    email_id_to_check_string = String::new();
                    io::stdin().read_line(&mut email_id_to_check_string).unwrap();
                    email_id_to_check_string = email_id_to_check_string.trim().to_string();
                    email_id_to_check = email_id_to_check_string.parse::<u32>().unwrap();
                }

                let api_url = format!("https://www.1secmail.com/api/v1/?action=readMessage&login={id}&domain={domain}&id={email_id}",
                    id = email_address.get_id(),
                    domain = email_address.get_domain(),
                    email_id = email_id_to_check
                );

                let response = get_api_data(&api_url).await;

                match response{
                    Ok(response) =>{
                        let response_email: FullEmail = response.json().await.expect("Should follow format of API");

                        println!("{}", response_email.to_string());
                    },
                    Err(err) => {
                        println!("Error with the API for getting content of email");
                        print!("Error: ");
                        println!("{}", err.to_string());
                        println!();
                    }
                }
            }

            "list" => {
                let mut count = 1;
                
                for email in &emails{
                    print!("{}", count.to_string() + ". ");
                    println!("{}", email.get_email());
                    count += 1;
                }

                println!();
            },
            
            "remove" => {
                let mut email_to_delete = "".to_string();
                let mut found_email = false;
                
                if command.len() > 1{
                    email_to_delete = command[1].to_string();
                } else{
                    email_to_delete = String::new();
                    io::stdin().read_line(&mut email_to_delete).unwrap();
                    email_to_delete = email_to_delete.trim().to_string();
                } 

                let mut index = 0;

                for email in &emails{
                    while !found_email{
                        if email.get_email() == email_to_delete{
                            found_email = true;
                        } else{
                            index += 1;
                        }
                    }
                }
                
                if found_email{
                    print!("Removed: ");
                    println!("{}", email_to_delete);
                    println!();
                    emails.swap_remove(index);
                } else{
                    println!("No email id of that type was found. Make sure the email id was typed.");
                    println!();
                }
            },
            "save_email" => save_email(),//To Code
            "help" => help(),
            "clear" => clear_terminal(),
            _ => {
                println!("Command doesn't exist, type help to get a list of commands");
                println!();
            }
        }
    }
}

fn help(){
    println!("List of commands:");
    println!("1. exit -> stops the program");
    println!("2. new -> generates a new email id");
    println!("3. list -> lists all currently generated email ids");
    println!("4. check_all -> lists all unread emails");
    println!("5. check_email_id OR check_email_id=[...] -> lists all unread emails from a specific email id");
    println!("6. remove OR remove=[...] -> removes an email id from the list of currently generated email ids");
    println!("7. save_email -> saves the content of the email to the system");
    println!("8. clear -> clears the current terminal");
    println!("9. check_email -> check the contexts of the email based on the given id");
    println!("10. clear -> clears the current terminal");
    println!();
    println!("Where there is a command with command=[...] the [...] can be replaced with the desired input to 
streamline the process. For example with remove_email_id the command remove_email_id.example@example.com 
would fulfil the functionality of remove_email_id without requiring a second step for the input where the 
email id would have been inputed");
    println!();
}

fn check_emails(){println!("check_emails")}
fn save_email(){println!("save_email")}

//API Functions
async fn get_api_data(url: &str) -> Result<Response, reqwest::Error> {
    // Make a GET request to the provided URL
    let response = reqwest::get(url).await?;

    Ok(response)
}

fn clear_terminal(){
    println!("{esc}c", esc = 27 as char);
}

async fn print_emails_from_id(email: Email){
    let api_url = format!("https://www.1secmail.com/api/v1/?action=getMessages&login={id}&domain={domain}",
                        id = email.get_id(),
                        domain = email.get_domain()
                    );

                    println!("{}", email.get_email());

                    let response = get_api_data(&api_url).await;

                    match response{
                        Ok(response) =>{
                            let response_emails: Vec<EmailInfo> = response.json().await.expect("Should follow format of API");

                            for response_email in &response_emails{
                                println!("{}", response_email.to_string());
                                println!();
                            }
                        },
                        Err(err) => {
                            println!("Error with the API for getting emails");
                            print!("Error: ");
                            println!("{}", err.to_string());
                            println!();
                        }
                    }
}