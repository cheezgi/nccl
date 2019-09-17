
use error::{NcclError, ErrorKind};
//use value::Value;
use ::TryInto;

use std::ops::{Index, IndexMut};
use std::error::Error;

/// Struct that contains configuration information.
///
/// Examples:
///
/// ```
/// let p = nccl::parse_file("examples/config.nccl").unwrap();
/// let ports = p["server"]["port"].keys_as::<i64>().unwrap();
///
/// println!("Operating on ports:");
/// for port in ports.iter() {
///     println!("  {}", port);
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Pair<'a> {
    key: &'a str,
    value: Vec<Pair<'a>>,
}
//pub struct Pair {
//    key: Value,
//    value: Vec<Pair>
//}

impl<'a> Pair<'a> {
    /// Creates a new Pair.
    pub fn new() -> Self {
        Pair {
            key: "",
            value: vec![]
        }
    }

    pub fn new_from(value: &'a str) -> Self {
        Pair {
            key: value,
            value: vec![],
        }
    }

    /// Adds a value to a Pair.
    ///
    /// Examples:
    ///
    /// ```
    /// let mut p = nccl::Pair::new("hello");
    /// p.add(true);
    /// p.add("world");
    /// ```
    pub fn add(&mut self, value: &'a str) {
        self.value.push(Pair::new_from(value));
    }

    pub fn parse<T: std::str::FromStr>(&self) -> Result<T, NcclError> {
        if self.value.is_empty() {
            Err(NcclError::new(ErrorKind::IntoError, "could not parse value", 0))
        } else {
            self.value[0].parse::<T>().or(Err(NcclError::new(ErrorKind::IntoError, "could not parse value", 0)))
        }
    }

    pub fn push(&mut self, p: Pair<'a>) {
        self.value.push(p);
    }


    ///// Recursively adds a slice to a Pair.
    //pub fn add_slice(&mut self, path: &[&str]) {
    //    let s = self.traverse_path(&path[0..path.len() - 1]);
    //    if !s.has_key(&path[path.len() - 1]) {
    //        s.add(&path[path.len() - 1]);
    //    }
    //}

    ///// Adds a Pair to a Pair.
    //pub fn add_pair(&mut self, pair: Pair) {
    //    if self.value.contains(&pair) {
    //        self.value.push(pair);
    //    } else {
    //        self[&pair.key].value = pair.value;
    //    }
    //}

    /// Test if a pair has a key.
    ///
    /// Examples:
    ///
    /// ```
    /// use nccl::NcclError;
    /// let mut p = nccl::parse_file("examples/config.nccl").unwrap();
    /// assert!(p.has_key("server"));
    /// assert!(p["server"]["port"].has_key(80));
    /// ```
    pub fn has_key(&self, key: &str) -> bool {
        //let k = key.into();
        for item in &self.value {
            if item.key == key {
                return true;
            }
        }

        false
    }

    /// Test if a pair has a path of values. Use `vec_into!` to make
    /// this method easier to use.
    ///
    /// Examples:
    ///
    /// ```
    /// # #[macro_use] extern crate nccl; fn main() {
    /// let mut p = nccl::parse_file("examples/config.nccl").unwrap();
    /// assert!(p.has_path(vec_into!["server", "port", 80]));
    /// # }
    /// ```
    pub fn has_path(&self, path: &[&str]) -> bool {
        if path.len() == 0 {
            true
        } else {
            if self.has_key(&path[0]) {
                self[&path[0]].has_path(&path[1..path.len()])
            } else {
                false
            }
        }
    }

    ///// Traverses a Pair using a slice, adding the item if it does not exist.
    //pub fn traverse_path(&mut self, path: &[&str]) -> &mut Pair {
    //    if path.is_empty() {
    //        &mut self
    //    } else {
    //        if !self.has_key(&path[0]) {
    //            self.add(&path[0]);
    //        }
    //        self[&path[0]].traverse_path(&path[1..path.len()])
    //    }
    //}

    ///// Gets a child Pair from a Pair. Used by Pair's implementation of Index.
    /////
    ///// ```
    ///// let mut p = nccl::Pair::new("top_level");
    ///// p.add("hello!");
    ///// p.get("hello!").unwrap();
    ///// ```
    //pub fn get(&mut self, value: T) -> Result<&mut Pair, Box<dyn Error>> where Value: From<T> {
    //    let v = value.into();

    //    if self.value.is_empty() {
    //        return Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, &format!("Pair does not have key: {}", v), 0)));
    //    } else {
    //        for item in &mut self.value {
    //            if item.key == v {
    //                return Ok(item);
    //            }
    //        }
    //    }

    //    Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, &format!("Could not find key: {}", v), 0)))
    //}

    ///// Gets a mutable child Pair from a Pair. Used by Pair's implementation of
    ///// IndexMut.
    /////
    ///// ```
    ///// let mut p = nccl::Pair::new("top_level");
    ///// p.add(32);
    ///// p.get(32).unwrap();
    ///// ```
    //pub fn get_ref<T>(&self, value: T) -> Result<&Pair, Box<dyn Error>> where Value: From<T> {
    //    let v = value.into();

    //    if self.value.is_empty() {
    //        return Ok(self);
    //    } else {
    //        for item in &self.value {
    //            if item.key == v {
    //                return Ok(item);
    //            }
    //        }
    //    }

    //    Err(Box::new(NcclError::new(ErrorKind::KeyNotFound, &format!("Could not find key: {}", v), 0)))
    //}

    /// Returns the value of a pair as a string.
    /// ```
    /// let config = nccl::parse_file("examples/long.nccl").unwrap();
    /// assert_eq!(config["bool too"].value().unwrap(), "false");
    /// ```
    pub fn value(&self) -> Option<String>  {
        if self.value.len() == 1 {
            Some(format!("{}", self.value[0].key.clone()))
        } else {
            None
        }
    }

    /// Returns the value of the key or a default value.
    pub fn value_or(&self, or: String) -> String {
        self.value().unwrap_or(or)
    }

    fn value_raw(&self) -> Option<String> {
        if self.value.len() == 1 {
            Some(self.value[0].key.to_string())
        } else {
            None
        }
    }

    ///// Gets the value of a key as a specified type, if there is only one.
    /////
    ///// Examples:
    /////
    ///// ```
    ///// let p = nccl::parse_file("examples/long.nccl").unwrap();
    ///// assert!(!p["bool too"].value_as::<bool>().unwrap());
    ///// ```
    //pub fn value_as<T>(&self) -> Result<T, Box<dyn Error>> {
    //    match self.value_raw() {
    //        Some(v) => match v.try_into() {
    //            Ok(t) => Ok(t),
    //            Err(_) => return Err(Box::new(NcclError::new(ErrorKind::IntoError, "Could not convert to T", 0)))
    //        },
    //        None => Err(Box::new(NcclError::new(ErrorKind::MultipleValues, "Could not convert value: multiple values. Use keys() or keys_as()", 0)))
    //    }
    //}

    ///// Gets the value of a key as a specified type or a default value.
    //pub fn value_as_or<T>(&self, or: T) -> T where Value: TryInto<T> {
    //    self.value_as::<T>().unwrap_or(or)
    //}

    fn keys(&self) -> Vec<String> {
        self.value.clone().into_iter().map(|x| x.key.to_string()).collect()
    }

    /// Gets keys of a value as a vector of T.
    ///
    /// Examples:
    ///
    /// ```
    /// let config = nccl::parse_file("examples/config.nccl").unwrap();
    /// let ports = config["server"]["port"].keys_as::<i64>().unwrap();
    /// assert_eq!(ports, vec![80, 443]);
    /// ```
    pub fn keys_as<T: std::str::FromStr>(&self) -> Result<Vec<T>, Box<dyn Error>> {
        let mut v: Vec<T> = vec![];
        for key in self.keys() {
            match key.parse::<T>() {
                Ok(k) => v.push(k),
                Err(_) => return Err(Box::new(NcclError::new(ErrorKind::IntoError, "Could not convert to T", 0)))
            }
        }
        Ok(v)
    }

    ///// Gets keys of a value as a vector of T or returns a default vector.
    //pub fn keys_as_or<T>(&self, or: Vec<T>) -> Vec<T> {
    //    self.keys_as::<T>().unwrap_or(or)
    //}

    /// Pretty-prints a Pair.
    ///
    /// Examples:
    ///
    /// ```
    /// let config = nccl::parse_file("examples/config.nccl").unwrap();
    /// config.pretty_print();
    ///
    /// // String("__top_level__")
    /// //     String("server")
    /// //         String("domain")
    /// //             String("example.com")
    /// //             String("www.example.com")
    /// //         String("port")
    /// //             Integer(80)
    /// //             Integer(443)
    /// //         String("root")
    /// //             String("/var/www/html")
    /// ```
    ///
    pub fn pretty_print(&self) {
        self.pp_rec(0);
    }

    fn pp_rec(&self, indent: u32) {
        for _ in 0..indent {
            print!("    ");
        }
        println!("{:?}", self.key);
        for value in &self.value {
            value.pp_rec(indent + 1);
        }
    }
}

impl<'a> Index<&str> for Pair<'a> {
    type Output = Pair<'a>;
    fn index(&self, i: &str) -> &Self::Output {
        for pair in self.value.iter() {
            if pair.key == i {
                return &pair;
            }
        }
        panic!("pair does not contain key: {}", i);
    }
}

impl<'a> IndexMut<&str> for Pair<'a> {
    fn index_mut(&mut self, idx: &str) -> &mut Pair<'a> {
        for mut pair in &mut self.value {
            if pair.key == idx {
                return pair;
            }
        }
        panic!("pair does not contain key: {}", idx);
    }
}
