use std::collections::HashMap;

use crate::mndb::drivers::mssql::Mssql;



pub type Row        = Vec<Value>;
pub type SchemaMap  = HashMap<String, TableSchema>;
pub type BoxError   = Box<dyn std::error::Error + Send + Sync>;



pub enum Dbs {
    
    SqlServer,
    //Postgresql,
}

pub enum Value {

    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

#[derive(Debug)]
pub struct TableInfo {
    
    pub schema          : String,
    pub name            : String,
    pub column          : String,
    pub data_type       : String,
    pub char_max_lenght : Option<i32>,
    pub is_null         : String,
    pub column_default  : String,
}

#[derive(Debug)]
pub struct KeyfInfo {
    
    pub keyfname        : String,
    pub schema_origin   : String,
    pub table_origin    : String,
    pub column_origin   : String,
    pub schema_dest     : String,
    pub table_dest      : String,
    pub column_dest     : String,
}

#[derive(Debug)]
pub struct KeypInfo {
    
    pub schema      : String,
    pub table       : String,
    pub column      : String,
    pub constraint  : String
}

#[derive(Debug, Clone, Default)]
pub struct ColumnInfo {

    pub name:            String,
    pub data_type:       String,
    pub char_max_lenght: Option<i32>,
    pub is_null:         String,
    pub column_default:  String,
}

#[derive(Debug, Clone)]
pub struct ForeignKey {

    pub name:          String,
    pub column:        String,
    pub schema_dest:   String,
    pub table_dest:    String,
    pub column_dest:   String,
}

#[derive(Debug, Clone, Default)]
pub struct TableSchema {

    pub schema:       String,
    pub columns:      Vec<ColumnInfo>,
    pub primary_keys: Vec<String>,        
    pub pk_constraint: Option<String>,    
    pub foreign_keys: Vec<ForeignKey>,
}



pub trait Database {

    fn query(&mut self, sql: &str)    -> Result<Vec<Row>, BoxError>;

    fn sql_tables(&self)        -> &'static str;
    fn sql_primary_keys(&self)  -> &'static str;
    fn sql_foreign_keys(&self)  -> &'static str;


    fn tables(&mut self) -> Result<Vec<TableInfo>, BoxError> {
        
        let rows = self.query(self.sql_tables())?;
        Ok(rows.iter().map(|r| TableInfo {
            schema:          r[0].as_str(),
            name:            r[1].as_str(),
            column:          r[2].as_str(),
            data_type:       r[3].as_str(),
            char_max_lenght: r[4].as_i32(),
            is_null:         r[5].as_str(),
            column_default:  r[6].as_str(),
        }).collect())
    }

    fn primary_keys(&mut self) -> Result<Vec<KeypInfo>, BoxError> {
        
        let rows = self.query(self.sql_primary_keys())?;
        Ok(rows.iter().map(|r| KeypInfo {
            schema:     r[0].as_str(),
            table:      r[1].as_str(),
            column:     r[2].as_str(),
            constraint: r[3].as_str(),
        }).collect())
    }

    fn foreign_keys(&mut self) -> Result<Vec<KeyfInfo>, BoxError> {
        
        let rows = self.query(self.sql_foreign_keys())?;
        Ok(rows.iter().map(|r| KeyfInfo {
            keyfname:      r[0].as_str(),
            schema_origin: r[1].as_str(),
            table_origin:  r[2].as_str(),
            column_origin: r[3].as_str(),
            schema_dest:   r[4].as_str(),
            table_dest:    r[5].as_str(),
            column_dest:   r[6].as_str(),
        }).collect())
    }   

    fn schema_map(&mut self) -> Result<SchemaMap, BoxError> {

        let mut map: SchemaMap = HashMap::new();

        for t in self.tables()? {
            let entry = map.entry(t.name.clone()).or_default();
            entry.schema = t.schema;
            entry.columns.push(ColumnInfo {
                name:            t.column,
                data_type:       t.data_type,
                char_max_lenght: t.char_max_lenght,
                is_null:         t.is_null,
                column_default:  t.column_default,
            });
        }

        for pk in self.primary_keys()? {
            if let Some(t) = map.get_mut(&pk.table) {
                t.primary_keys.push(pk.column);
                t.pk_constraint = Some(pk.constraint);
            }
        }

        for fk in self.foreign_keys()? {
            if let Some(t) = map.get_mut(&fk.table_origin) {
                t.foreign_keys.push(ForeignKey {
                    name:        fk.keyfname,
                    column:      fk.column_origin,
                    schema_dest: fk.schema_dest,
                    table_dest:  fk.table_dest,
                    column_dest: fk.column_dest,
                });
            }
        }

        Ok(map)
    }
}

impl Value {
    
    pub fn as_str(&self) -> String {
        
        match self {
            Value::Text(s) => s.clone(),
            _ => String::new()
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        
        match self {
            Value::Int(i) => Some(*i as i32),
            _ => None
        }

    }
}

impl Dbs {
    
    pub fn connect(&self) -> Result<Box<dyn Database>, BoxError>  {
        
        match self {

            Dbs::SqlServer => Ok(Box::new(Mssql::connect()?)),
        }

    }

}
