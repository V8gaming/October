# October Language App

A language learning app written in rust. Only testing memory for now.

## Usage

```powershell
cargo run --release
```

## Language Config
alphabet options are: "english", "vietnamese", "greek", and "cyrillic".
## Todo

- [x] Convert Sandbox to Application
- [x] Keypresses
- [x] Timer
- [ ] Sound
- [x] Main page
- [x] Remake database as (table name = level. data = [english, vietnamese, type])
- [x] Language Page
  - [x] change loaded tables based on database loaded
  - [x] write enums and tables based on min and max size of amount of tables in databas (idea use numbers in code and index the list using that number)
- [ ] Options Page
  - [x] Text size
  - [ ] Sound or no sound
  - [ ] Check for both meanings seperated by a semicolon, or check for either
  - [x] Volume
  - [x] Timed
  - [x] length of timer
- [x] languages as folder(database, settings.toml)
- [x] Title in main page, and selected language.
- [ ] Add comments to code.
- [ ] red theme
- [x] Review Page
- [ ] Compile for mac and linux
- [ ] Check for both meanings seperated by a semicolon, or check for either (requires settings)
- [ ] allow multiple database types [sqlite3, sqlite]
- [ ] add tests for Message Matching
- [ ] change loadsettings to a hashmap

## Future

- [ ] hot loading of languages.
- [x] Compile without generator
- [ ] No Vecs
- [ ] No unwraps
