use crate::mndb::drivers::mssql::Mssql;


pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct TableInfo {
    
    pub schema  : String,
    pub name    : String,
}

#[derive(Debug)]
pub struct KeyfInfo {
    
    pub table  : String,
    pub column    : String,
    pub name    : String,
}

#[derive(Debug)]
pub struct KeypInfo {
    
    pub table  : String,
    pub column    : String,
    pub name    : String,
}

#[derive(Debug)]
pub struct SchemaInfo {
    
    pub tables          : Vec<TableInfo>,
    // pub primary_keys    : Vec<KeypInfo>,
    // pub foreign_keys    : Vec<KeyfInfo>,
}

pub trait Database {

    fn tables(&mut self)        -> Result<Vec<TableInfo>, BoxError>;
    // fn primary_keys(&mut self)  -> Result<Vec<KeypInfo>, BoxError>;
    // fn foreign_keys(&mut self)  -> Result<Vec<KeyfInfo>, BoxError>;

    fn schema(&mut self) -> Result<SchemaInfo, BoxError> {

        Ok(SchemaInfo {
            tables: self.tables()?,
            // primary_keys: self.primary_keys()?,
            // foreign_keys: self.foreign_keys()?,
        })
    }
}

pub enum Dbs {
    
    SqlServer,
    //Postgresql,
}

impl Dbs {
    
    pub fn connect(&self) -> Result<Box<dyn Database>, BoxError>  {
        
        match self {

            Dbs::SqlServer => Ok(Box::new(Mssql::connect()?)),
        }

    }

}
