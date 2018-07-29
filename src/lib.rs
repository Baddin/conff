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

  

    pub fn write_to_file(&self){
        match &self.cnftype {
            ConfType::Toml => self.write_data_to_toml(),
            ConfType::Yaml => self.write_data_to_yaml(),
            ConfType::Json => self.write_data_to_json(),
            ConfType::Csv => self.write_data_to_csv(),
            _ => self.write_data()
        };
    }

    fn write_data_to_toml(&self){ unimplemented!(); }
    fn write_data_to_yaml(&self){ unimplemented!(); }
    fn write_data_to_json(&self){ unimplemented!(); }
    fn write_data_to_csv(&self){ unimplemented!(); }
    fn write_data(&self){ unimplemented!(); }


}

enum ConfType{
    Toml,
    Yaml,
    Json,
    Csv,
    Other
}


