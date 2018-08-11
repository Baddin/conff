extern crate csv;
use conf::Conf;
use std::path::Path;
use std::fs;
//use std::io::prelude::*;




struct CsvConf{
    filename: String,
    data: Vec<Vec<String>>
}

impl CsvConf{
    fn new(filename: String, data: Vec<Vec<String>>)-> CsvConf{
          CsvConf {
            filename: filename,
            data: data
        } 
    } 
  
}

impl Conf for CsvConf {
    
    /*
    fn change_data(&mut self, records:Vec<String>){
	unimplemented!();
  
    }
     */
    /*
    fn append_data(&mut self, name: String, value:String){
        &self.data.push(format!("{}, {}", name, value));
    }
    */
    
   // fn extracted_data(&self)->HashMap<String, String>; //later
    fn write_to_file(&self){
        let mut writer = csv::Writer::from_path(&self.filename).unwrap();
        //writer.write_record(&["test1","test2","test3"]);
       // writer.write_record(&["tst1","tst2","tst3"]);
        for rec in &self.data{
            writer.write_record(rec);
        }
        writer.flush();

    }
}
//TODO: Write tests



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_write_to_file(){

        let d1:Vec<String> = vec!("value1".to_string(), "value2".to_string(), "value2".to_string());
        let d2:Vec<String> = vec!("value2.1".to_string(), "value2.2".to_string(), "value2.3".to_string());
        let d3:Vec<String> = vec!("value3.1".to_string(), "value3.2".to_string(), "value3.3".to_string());
        let data: Vec<Vec<String>> = vec!(d1, d2, d3);
        let filename = "test_csv.csv".to_string();
        let test_conf = CsvConf::new(filename.clone(), data);
        test_conf.write_to_file();
        assert_eq!(Path::new(&filename).exists(), true);
        fs::remove_file(&filename);
    }
}
