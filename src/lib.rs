use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct Conf{
    filename: String,
    cnftype: ConfType,
    data: HashMap<String, String> 
}

impl Conf {
    pub fn new(&self, filename: String, cnftype: ConfType, data: HashMap<String, String>) -> Conf{
        Conf {
            filename: filename,
            cnftype: cnftype,
            data:  data
        }
    }

    pub fn change_data(&mut self, name:String, value:String) {
        if let Some(_) = &self.data.get(&name) {
            &self.data.insert(name, value);
        }
    }

    pub fn append_data(&mut self, name:String, value: String){
        &self.data.insert(name, value);
    }

    fn extracted_data(&self)->HashMap<String, String>{
        unimplemented!(); //TODO: get the data from a confing file and put it in a hashmap
    }

    fn read_file(&self) -> Result<String, std::io::Error>{
        let mut f = File::open(&self.filename)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?; 
        Ok(content)
    }

    fn write_to_file(){
        unimplemented!();
    }

}

enum ConfType{
    Toml,
    Yaml,
    Json,
    Csv,
    Other
}


