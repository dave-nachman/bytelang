
use bytecode::Value;


pub fn pretty_print(value: &Value) -> String {
    match value {
        Value::Int(value) => {
             format!("{}",value)
        }
        Value::Float(value) => {
            format!("{}",value)
       }
       Value::Bool(value) => {
            format!("{}",value)
        }
        Value::Pointer(_) => {
            "<pointer>".to_owned()
        }
        Value::Bytes(_) => {
            "<bytes>".to_owned()
        }
        Value::Function(_, _, _) => {
            "<function>".to_owned()
        }
      
    
    }
   
}