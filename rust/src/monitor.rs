use std::fs::File;
use std::path::Path;

pub struct Monitor<T> {
    name: String,
    value: *const T
}

impl<T> Monitor<T> {
    pub fn new(name: String, value: &T) -> Monitor<T> {
        Monitor{
            name: name,
            value: value as *const T
        }
    }

    pub fn to_string(&self) -> String {
        let mut monitor_string = String::from(self.name);
        monitor_string.push_str(": ");
        monitor_string.push_str(&self.value.to_string());
        return monitor_string;
    }
}

pub struct MonitorArrayWriter<'a, T> {
    monitors: Vec<Monitor<T>>,
    path: &'a Path,
    file: File
}

impl<'a, T> MonitorArrayWriter<'a, T> {
    pub fn new(monitors: Vec<Monitor<T>>, path: &'a Path) -> MonitorArrayWriter<'a, T> {
        // Init file connection
        let file = MonitorArrayWriter::get_file_connection(path);
        MonitorArrayWriter {
            monitors: monitors,
            path: &path,
            file: file
        }
    }

    fn get_file_connection(path: &Path) -> File {
        let file = match File::create(path) {
            Err(why) => panic!("Couldn't create file {}: {}", path.display(), why),
            Ok(file) => file
        };
        return file;
    }

    pub fn write(&self) {
        let mut line = String::from("{");
        for mon in &self.monitors {
            line.push_str(&mon.to_string());
            line.push_str(", ");
        }
        line.truncate(line.len() - 2); // Strip trailing comma.
        line.push_str("},\n");
    }

    pub fn close(&self) {
        //Write }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_changing_variable() {
        let mut changing_variable : f32 = 10.0;
        let monitor = Monitor {
            name: String::from("testvar"),
            value: &mut changing_variable
        };
        changing_variable = 20.0;
        assert_eq!(*monitor.value, 20.0, "Monitor value does not change with target variable.")
    }
}
