# Cringe Word
It's a basic game where a word (with shuffled letters) will be given and you have guess what it is.

## Building from source
* clone this repo
    ```bash
    git clone https://github.com/PrajwalCH/cringe_word_rem
    cd cringe_word_rem
    ```
* place the `words_data.txt` file on your _HOME_ dir.
    ```bash
    cp/mv words_data.txt $HOME
    ```
    It's important to have the same name and location of file. You can append the new word on data file and placing it to new line.

* Finally compile and run
    ```bash
    cargo build
    cargo run

    # or directly
    cargo run
    ```

