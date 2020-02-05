pub struct Monitor {
    name: String,
    value: f32
}

impl Monitor {
    pub fn to_string(&self) -> String {
        let mut monitor_string = String::from(self.name);
        monitor_string.push_str(": ");
        monitor_string.push_str(String::from(self.value));
        return monitor_string;
    }
}

pub struct MonitorArrayWriter{
    monitors: Vec<&Monitor>,
    file: ??? //TODO: figure out how to attach file.
}

impl MonitorArrayWriter{
    pub fn new(monitors: Vec<&Monitor>, file: ???) -> MonitorArrayWriter {
        // Init file connection
        MonitorArrayWriter {
            monitors: monitors,
            file: file
        }
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


