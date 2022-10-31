# October Language App

A language learning app written in rust. Only testing memory for now.

## Usage

```powershell
cargo run --bin generator; cargo run --bin generated
```

## Todo

- [x] Convert Sandbox to Application
- [x] Keypresses
- [ ] Timer
- [ ] Sound
- [x] Main page
- [x] Remake database as (table name = level. data = [english, vietnamese, type])
- [x] Language Page 
    - [x] change loaded tables based on database loaded
    - [x] write enums and tables based on min and max size of amount of tables in databas (idea use numbers in code and index the list using that number)
- [ ] Options Page
    - [ ] Text size
    - [ ] Sound or no sound
- [ ] languages as folder(database, settings.TOML)
- [ ] Title in main page
- [ ] red there
- [x] Review Page

## Future

- [ ] *do this without generator, for hot loading of languages.
- [ ] No Vecs
- [ ] Compile without generator
- [ ] No unwraps
