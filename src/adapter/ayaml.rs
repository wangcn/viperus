use super::*;
use serde_yaml;

/// YamlAdapter map a Yaml file in a linear multilevel key/value array
/// 
/// the adaptor could be consumed by Viperous  
pub struct YamlAdapter {
    source: String,
    data: serde_yaml::Mapping,
    //config_map: crate::map::Map,
}


impl YamlAdapter {
    pub fn new() -> Self {
        YamlAdapter {
            source: String::default(),
            data: serde_yaml::Mapping::new(),
        }
    }

    /// load_file 
    /// 
    /// # Arguments
    /// * `name`
    pub fn load_file(&mut self, name: &str) -> AdapterResult<()> {
        self.source = std::fs::read_to_string(name)?;

        Ok(())
    }

    pub fn load_str(&mut self, source: &str) -> AdapterResult<()> {
        self.source = source.to_owned();

        Ok(())
    }


}

impl ConfigAdapter for YamlAdapter {
    fn parse(&mut self) -> AdapterResult<()> {
        self.data = serde_yaml::from_str::<serde_yaml::Mapping>(&self.source)?;

        Ok(())
    }

    fn get_map(&self) -> crate::map::Map {
        let mut res = crate::map::Map::new();

        //let mut kpath;

        for (k, v) in self.data.iter() {
            if let serde_yaml::Value::String(s) = k {
                let kpath = s.to_owned();

                rec_yaml(&mut res, &kpath, &v);
            }
        }

        res
    }
}

fn rec_yaml(config_map: &mut crate::map::Map, kpath: &str, v: &serde_yaml::Value) {
    debug!("{:?} => {:?}", kpath, v);

    match v {
        serde_yaml::Value::Mapping(m) => {
            for (kk, vv) in m {
                if let serde_yaml::Value::String(s) = kk {
                    let kk = format!("{}.{}", kpath, s);
                    rec_yaml(config_map, &kk, vv);
                }
            }
        }

        serde_yaml::Value::Sequence(m) => {
            for vv in m {
                let kk = kpath.to_string();
                rec_yaml(config_map, &kk, vv);
            }
        }
        serde_yaml::Value::String(s) => { 
            config_map.add(kpath, s.clone());
        }

        serde_yaml::Value::Number(num) => {
            let i= num.as_i64().unwrap_or_default() as i32;

            config_map.add(kpath, i);
        }


        serde_yaml::Value::Bool(b) => {
            config_map.add(kpath, *b);
        }

        _ => (),
    }
}