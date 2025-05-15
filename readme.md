# dependences
- See Cargo.toml

# How to edit
- open main.rs in your text editor of choice.
- See main.rs function body examples
- the generator is empty by default, you can add parts to by invoking add_part(key, vector of strings to be randomly chosen)
- - Note currently, add_part overrides existing part data.
- provide a template via add_template, e.g. "Hi {name}", if words in brackets match a provided key that was added it will randomly be replaced
with one of the permutation options included in add_part.
- .generator.unwrap() generates the procedurally generated description from the information provided.
- - Note both a part & template is required before calling this function or it will panic and crash
# How to build
- cargo build
# How to Run
- cargo run ( builds and runs it for you)
- or run the resulting executable from cargo build(see target folder)
