pub trait Conf {
   // fn new(filename: String, data: T) -> Self ;
    //fn change_data(&mut self, name:String, value:String);
    //fn append_data(&mut self, name:String, value: String);
   // fn extracted_data(&self)->HashMap<String, String>; //later
    fn write_to_file(&self);
}