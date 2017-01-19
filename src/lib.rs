// I don't really know what belongs here yet, but this should be enough to get started.
pub struct BackEnd {
    buffers: Vec<Buffer>,
    current_index: i32,
}

impl BackEnd {
    pub fn new() -> BackEnd {
        BackEnd {
            buffers: vec![Buffer::empty()],
            current_index: 0,
        }
    }
}

// Same here
pub struct Buffer {
    source: String,
    filename: Option<String>,
}

impl Buffer {
    pub fn new(source: String, filename: Option<String>) -> Buffer {
        Buffer {
            source: source,
            filename: filename,
        }
    }
    // TODO
    pub fn from_file(filename: String) -> Buffer {
        Buffer::new(String::new(), Some(filename))
    }
    pub fn empty() -> Buffer {
        Buffer::new(String::new(), None)
    }
}


// I need to write some tests...
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
