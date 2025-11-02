use std::{collections::BTreeMap, io::BufWriter};

use mcap::{Writer, records::MessageHeader};

const SCHEMA_DATA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "value": {
      "type": "number"
    }
  },
  "required": [
    "value"
  ],
  "additionalProperties": false
}"#;

fn main() {
    println!("Hello, world!");

    let file = std::fs::File::create("example.mcap").unwrap();
    let file = BufWriter::new(file);

    let mut writer = Writer::new(file).unwrap();

    let schema_id = writer
        .add_schema("my-schema", "jsonschema", SCHEMA_DATA.as_bytes())
        .unwrap();

    let channel_id = writer
        .add_channel(schema_id, "my topic", "json", &BTreeMap::new())
        .unwrap();

    writer
        .write_to_known_channel(
            &MessageHeader {
                channel_id,
                sequence: 1,
                log_time: 1,
                publish_time: 1,
            },
            b"{ 
                \"value\": 1 
            }",
        )
        .unwrap();

    writer.finish().unwrap();
}
