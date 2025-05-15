use std::{collections::HashMap, fs::File};
use rand::seq::{IndexedRandom};
use regex::Regex;
use rand::rng;
use serde::{Serialize,Deserialize};
use std::io::Write;
use std::error::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Generator {
    pub parts: HashMap<String, Vec<String> >,
    pub templates: Vec<String>,
}

impl Generator{
    pub fn add_part(&mut self, key: &str, options: Vec<&str> ){
        self.parts.insert(key.to_string(), options.iter().map(|s| s.to_string()).collect() );
    }
    pub fn add_template(&mut self, template: &str ){
        self.templates.push( template.to_string() );
    }
    // ? Consider adding a "feature flag", for binary support later, TOML might be too costly at scale all the time.
    pub fn import_toml(&self, path: &str)-> Result<Generator, Box<dyn Error> >{
        let contents = std::fs::read_to_string(path)?;
        let toml_obj = toml::from_str(&contents)?;
        Ok(toml_obj)
    }
    pub fn export_toml(&self, path : &str)->std::io::Result<()>{
        let toml  = toml::to_string(&self).unwrap();
        let mut output = File::create(path)?;
        write!(output, "{}", toml)?;
        Ok(())
    }
    pub fn generate(&self)-> Option<String>{
        let mut rng = rng();
        let template = self.templates.choose(&mut rng)?;
        let result = Regex::new(r"\{(\w+)\}").unwrap().replace_all(template,
        |caps: &regex::Captures| {
             let key = &caps[1];
             if let Some(choices) = self.parts.get(key){
                choices.choose(&mut rng).cloned().unwrap_or_else(|| format!("{{{}}}", key ) )
             } else{
                format!("{{{}}}", key )
             }
        });
        return Some( result.into_owned() )
    }
}