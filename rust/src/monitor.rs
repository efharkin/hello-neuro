use std::fs::File;
use std::path::Path;

pub struct Monitor<'a> {
    name: String,
    value: &'a mut f32
}

impl Monitor<'_> {
    pub fn to_string(&self) -> String {
        let mut monitor_string = String::from(self.name);
        monitor_string.push_str(": ");
        monitor_string.push_str(String::from(self.value));
        return monitor_string;
    }
}

pub struct MonitorArrayWriter {
    monitors: Vec<Monitor> where 'a Monitor,
    path: Path,
    file: File
}

impl MonitorArrayWriter<'_> {
    pub fn new(monitors: Vec<Monitor>, path: Path) -> MonitorArrayWriter {
        // Init file connection
        let mut file = MonitorArrayWriter::get_file_connection(path);
        MonitorArrayWriter {
            monitors: monitors,
            path: path,
            file: file
        }
    }

    fn get_file_connection(path: Path) -> File {
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create file {}: {}", path.display(), why.description()),
            Ok(file) => file
        };
        return file;
    }

    pub fn write(&self) {
        let mut line = String::from("{");
        for mon in self.monitors {
            line.push_str(mon.to_string);
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
            name: "testvar",
            value: &changing_variable
        };
        changing_variable = 20.0;
        assert_eq!(monitor.value, 20, "Monitor value does not change with target variable.")
    }
}
