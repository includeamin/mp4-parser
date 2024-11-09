# mp4-parser

**mp4-parser** is a Rust-based, open-source library focused on exploring and implementing the MP4 (MPEG-4 Part 14) file format based on key RFC standards (RFC 4337, RFC 6381, and RFC 6416). This library aims to parse MP4 container formats, enabling developers to work directly with the structure and data of MP4 files, including metadata, audio, and video streams.

## Goals and To-Do List

### Project Goals:
- **Standard-Conformant Parsing**: Implement a parser that interprets MP4 files based on standardized RFC specifications, with a focus on MIME types, codecs, and payload formats.
- **Accessible API for Rust Projects**: Provide a Rust-native API for developers and researchers to read, analyze, and manipulate MP4 files within Rust projects.

### To-Do List:
- [ ] **File Header Parsing**: Parse the primary file headers to extract information like file type, compatibility, and codec data.
- [ ] **Metadata Extraction**: Access and interpret metadata to retrieve duration, resolution, codec information, and other descriptive fields.
- [ ] **Audio/Visual Stream Access**: Implement the ability to identify, parse, and access MPEG-4 audio and video streams according to RFC 6416 specifications.
- [ ] **Error Handling and Validation**: Develop robust error handling to ensure resilience to malformed files and partial MP4s.

## Important Notes

1. **Experimental Implementation**: This is an early-stage, experimental library, intended primarily as an exploration of MP4 specifications. It may not fully comply with all aspects of the standards yet.
2. **Educational Focus**: The main objective is to provide a learning tool for those interested in understanding MP4 internals, with a roadmap to further compliance and usability.

## Supported Standards
- **RFC 4337**: MPEG-4 MIME Type Registration
- **RFC 6381**: Codec Parameters for MP4 Media Types
- **RFC 6416**: RTP Payload Format for MPEG-4 Streams

## Example Usage (Prototype)

The following example shows how the library may be used in the future to parse an MP4 file and access basic metadata:

```rust
use mp4_parser::Mp4Parser;

fn main() {
    let parser = Mp4Parser::new("sample.mp4");
    match parser.parse() {
        Ok(metadata) => println!("MP4 Metadata: {:?}", metadata),
        Err(e) => println!("Error parsing MP4 file: {:?}", e),
    }
}
```

## Contributing

We welcome contributions! If you have ideas, bug fixes, or enhancements, please fork the repository and submit a pull request.

## License

`mp4-parser` is licensed under the MIT License.
