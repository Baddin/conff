use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;



struct TextConf{
    filename: String,
    data: HashMap<String, String> 
}


trait Conf {
    fn new(filename: String, data: HashMap<String, String>) -> Self ;
    fn change_data(&mut self, name:String, value:String);
    fn append_data(&mut self, name:String, value: String);
    fn extracted_data(&self)->HashMap<String, String>;
    fn write_to_file(&self);
}


impl Conf for TextConf {
    fn new(filename: String, data: HashMap<String, String>) -> TextConf{
        TextConf {
            filename: filename,
            data: data
        }
    }

    fn change_data(&mut self, name:String, value:String) {
        if let Some(_) = &self.data.get(&name) {
            &self.data.insert(name, value);
        }
    }

    fn append_data(&mut self, name:String, value: String){
        &self.data.insert(name, value);
    }

    fn extracted_data(&self)->HashMap<String, String>{
        unimplemented!(); //TODO: get the data from a config file and put it in a hashmap
    }
    fn write_to_file(&self){
        let mut string_data = String::new();
        for (key, value) in &self.data{
            string_data += &format!("{} : {}\n", key, value);
        }
        
        fs::write(&self.filename, string_data ).expect("Could not write to the config file.")
    }
}






  
/*
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

    fn write_data(&self){
        let mut string_data = String::new();
        for (key, value) in &self.data{
            string_data += &format!("{} : {}\n", key, value);
        }
        
        fs::write(&self.filename, string_data ).expect("Could not write to the config file.")
    }


}


enum ConfType{
    Toml,
    Yaml,
    Json,
    Csv,
    Other
}

     */

/*
fn read_file(filename: String) -> Result<String, std::io::Error>{
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?; 
    Ok(content)
}
*/

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn test_write_data(){
        let filename = "test.cnf".to_string();
        let mut data =  HashMap::new();

        data.insert("test0".to_string(), "test".to_string());
        data.insert("test1".to_string(), "test".to_string());
        data.insert("test2".to_string(), "test".to_string());
        let test_conf:TextConf = Conf::new(filename.clone(), data.clone());
        test_conf.write_to_file();
        assert_eq!(Path::new(&filename).exists(), true);

        let mut f = File::open(&filename).expect("Could not open the config file.");
        let mut content = String::new();
        f.read_to_string(&mut content).expect("Could not read from the config file.");
        let mut string_data = String::new();
        for (key, value) in data{
            string_data += &format!("{} : {}\n", key, value);
        }
        assert_eq!(content, string_data);
        fs::remove_file(&filename);

    }

    #[test]
    fn test_append_data(){
        let mut data = HashMap::new();
        let mut test_conf: TextConf = Conf::new("test".to_string(), data);
        test_conf.append_data("test_key".to_string(), "test_value".to_string());
        assert_eq!(test_conf.data.get("test_key").unwrap(), &"test_value".to_string());
    }

    #[test]
    fn test_change_data(){
        let mut data = HashMap::new();
        data.insert("test_key".to_string(), "test_value".to_string());
        let mut test_conf: TextConf = Conf::new("test".to_string(), data);
        test_conf.change_data("test_key".to_string(), "test_value_changed".to_string());
        assert_eq!(test_conf.data.get("test_key").unwrap(), &"test_value_changed".to_string());
    }
}
