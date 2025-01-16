use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;

pub struct ItemDatabase {
    name: String,
    data: Vec<Item>,
}

impl ItemDatabase {
    pub fn new(name: String) -> Self {
        let mut new_database = Self {
            name,
            data: Vec::new(),
        };
        new_database.write_file();
        new_database
    }

    pub fn open(name: String) -> Self {
        let database = Self::read_file(name);
        database
    }

    pub fn add(&mut self, item_name: String, price: f32) {
        self.data.push(Item::new(item_name, price));
    }

    pub fn remove(&mut self, name: String) -> Result<(), String> {
        match self.search(name) {
            Some(index) => {
                let _output_item = self.data.remove(index as usize);
                Ok(())
            },
            None => Err(String::from("No item of this name found."))
        }
    }

    pub fn close(self) {
        self.write_file();
    }

    fn search(&self, try_name: String) -> Option<u64> {
        let mut index = 0;
        let mut found = false;
        for i in &self.data {
            if i.name == try_name {
                found = true;
                break;
            } else {
                index += 1;
            }
        }
        if found == true {
            Some(index)
        } else {
            None
        }
    }

    fn read_file(name: String) -> Self {
        let mut open_file = File::open(name).expect("No file by this name.");
        let mut unparsed_database = String::new();
        open_file.read_to_string(&mut unparsed_database);
        let database = Self::parse_database(unparsed_database);
        database
    }

    fn write_file(&self) {
        let mut open_file = File::open(&self.name).expect("No file by this name.");
        let deparsed_database = self.deparse_database();
        open_file.write_all(&deparsed_database.into_bytes());
    }

    fn parse_database(_unparsed_database: String) -> Self {
        let name = String::new();
        let data = Vec::new();

        // Parses String into Items.

        Self {
            name,
            data,
        }
    }

    fn deparse_database(& self) -> String {
        let mut output = String::new();
        for item in &self.data {
            output = output + &item.item_to_string();
        }
        output
    }
}



struct Item {
    name: String,
    price: f32,
    purchase_hist: Vec<String>,
}

impl Item {
    fn new(name: String, price: f32) -> Self {
        Self {
            name,
            price,
            purchase_hist: Vec::new(),
        }
    }

    fn bought_now(&mut self) {
        let mut now = Utc::now().to_string();
        now.truncate(10);
        self.purchase_hist.push(now + "--");
    }

    fn new_price(&mut self, new_price: f32) {
        self.price = new_price;
    }

    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn item_to_string(&self) -> String {
        let mut output = String::new();
        output = output + &self.name;
        output = output + ",";
        output = output + &self.price.to_string();
        output = output + ",";
        for purchase in &self.purchase_hist {
            output = output + &purchase;
            output = output + ",";
        }
        output
    }
}
