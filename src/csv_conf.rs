extern crate csv;
use conf::Conf;

struct CsvConf{
    filename: String,
    data: Vec<String>
}

impl CsvConf{
    fn new(filename: String, data: Vec<String>)-> CsvConf{
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
    
    fn append_data(&mut self, record: Vec<String>){
        unimplemented!();
    }
    */
   // fn extracted_data(&self)->HashMap<String, String>; //later
    fn write_to_file(&self){
        let mut writer = csv::Writer::from_path(&self.filename).unwrap();
        writer.write_record(&["test1","test2","test3"]);
        writer.write_record(&["tst1","tst2","tst3"]);
        writer.flush();

    }
}
//TODO: Write tests